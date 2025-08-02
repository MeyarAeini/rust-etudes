use failure::Fail;
use failure::ResultExt;
use rayon::prelude::*;
use rusoto_ec2::Ec2;
use std::path::Path;
use std::str::FromStr;
use std::{collections::HashMap, fmt::format};

mod ssh;

#[macro_use(defer)]
extern crate scopeguard;
extern crate failure;
extern crate rusoto;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_ec2;
extern crate ssh2;
//#[macro_use]
extern crate failure_derive;

//#[derive(Debug, Fail)]
//enum BurstError {
//}

//struct Burst {}

pub struct BurstBuilder {
    descriptors: std::collections::HashMap<String, (MachineSetup, i64)>,
    max_duration_time: i64,
}

impl Default for BurstBuilder {
    fn default() -> Self {
        Self {
            descriptors: Default::default(),
            max_duration_time: 60,
        }
    }
}

pub struct MachineSetup {
    instance_type: String,
    ami: String,
    setup: Box<dyn Fn(&mut ssh::Session) -> Result<(), failure::Error> + Sync>,
}

impl MachineSetup {
    pub fn new<F>(instance_type: &str, ami: &str, setup: F) -> Self
    where
        F: Fn(&mut ssh::Session) -> Result<(), failure::Error> + 'static + Sync,
    {
        Self {
            instance_type: instance_type.to_string(),
            ami: ami.to_string(),
            setup: Box::new(setup),
        }
    }
}

impl BurstBuilder {
    pub fn add_setup(&mut self, name: String, number: i64, setup: MachineSetup) {
        self.descriptors.insert(name, (setup, number));
    }

    pub fn set_max_duration_hour(&mut self, hour: u8) {
        self.max_duration_time = hour as i64 * 60;
    }

    fn rand_string(len: usize) -> String {
        use rand::{Rng, distr::Alphabetic};

        let mut rng = rand::rng();
        (&mut rng)
            .sample_iter(Alphabetic)
            .take(len)
            .map(char::from)
            .collect()
    }

    pub async fn run<F>(&mut self, mut script: F) -> Result<(), failure::Error>
    where
        F: FnMut(std::collections::HashMap<String, Vec<Machine>>) -> Result<(), failure::Error>,
    {
        use scopeguard::ScopeGuard;
        //Creates a client backed by the default tokio event loop.
        //The client will use the default credentials provider and tls client.
        let ec2 = rusoto_ec2::Ec2Client::new(rusoto_core::Region::UsEast1);

        //Create a security group
        let security_group_name = format!("burst_sg_{}", Self::rand_string(10));
        let sg_req = rusoto_ec2::CreateSecurityGroupRequest {
            group_name: security_group_name,
            description: "Security group for burst".to_string(),
            ..Default::default()
        };
        let group_id = ec2
            .create_security_group(sg_req)
            .await
            .context("failed to create security group")?
            .group_id
            .expect("aws creates security group always with an id");

        let ssh_permission = rusoto_ec2::IpPermission {
            from_port: Some(22),
            to_port: Some(22),
            ip_protocol: Some("tcp".to_string()),
            ip_ranges: Some(vec![rusoto_ec2::IpRange {
                cidr_ip: Some("0.0.0.0/0".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let tcp_cross_permission = rusoto_ec2::IpPermission {
            from_port: Some(0),
            to_port: Some(65535),
            ip_protocol: Some("tcp".to_string()),
            ip_ranges: Some(vec![rusoto_ec2::IpRange {
                cidr_ip: Some("172.31.0.0/16".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let sg_inbound_rule_req = rusoto_ec2::AuthorizeSecurityGroupIngressRequest {
            group_id: Some(group_id.clone()),
            ip_permissions: Some(vec![ssh_permission, tcp_cross_permission]),
            ..Default::default()
        };

        let _ = ec2
            .authorize_security_group_ingress(sg_inbound_rule_req)
            .await
            .context("Failed to setup security group permissions")?;

        //Create key pairs for ssh
        let key_name = format!("burst_{}", Self::rand_string(6));

        let key_pair_req = rusoto_ec2::CreateKeyPairRequest {
            key_name: key_name.clone(),
            ..Default::default()
        };

        let key_pair = ec2
            .create_key_pair(key_pair_req)
            .await
            .context("failed to generate the ec2 key-pairs")?;

        let key_pair_file =
            tempfile::NamedTempFile::new().context("failed to create a temp file")?;
        std::fs::write(
            key_pair_file.path(),
            key_pair
                .key_material
                .expect("the aws key-pair creation was successfull, so this should have value."),
        )
        .context(format!(
            "failed to store the key-pair private key file : {}",
            key_pair_file.path().to_str().unwrap()
        ))?;

        // 1. issue spot requests
        let mut id_to_name = HashMap::new();
        let mut spot_instance_request_ids = Vec::new();
        for (name, (setup, number)) in &self.descriptors {
            let launch = rusoto_ec2::RequestSpotLaunchSpecification {
                image_id: Some(setup.ami.clone()),
                instance_type: Some(setup.instance_type.clone()),
                security_group_ids: Some(vec![group_id.clone()]),
                key_name: Some(key_name.clone()),
                ..Default::default()
            };

            let req = rusoto_ec2::RequestSpotInstancesRequest {
                launch_specification: Some(launch),
                //block_duration_minutes: Some(self.max_duration_time),
                instance_count: Some(*number),
                //instance_interruption_behavior: Some("stop".to_string()),
                ..Default::default()
            };
            let result = ec2
                .request_spot_instances(req.clone())
                .await
                .map_err(failure::Error::from)
                .map_err(|e| e.context(format!("failed to request spot instances for {}", name)))?;

            let requests = result.spot_instance_requests.expect(
                "request spot instances should always return one or more spot instance requests",
            );
            spot_instance_request_ids.extend(
                requests
                    .into_iter()
                    .filter_map(|it| it.spot_instance_request_id)
                    .map(|it| {
                        id_to_name.insert(it.clone(), name.clone());
                        it
                    }),
            );
        }
        let mut desc_req = rusoto_ec2::DescribeSpotInstanceRequestsRequest::default();
        desc_req.spot_instance_request_ids = Some(spot_instance_request_ids);

        let mut all_active;
        let instance_ids: Vec<String> = loop {
            let result = ec2
                .describe_spot_instance_requests(desc_req.clone())
                .await
                .map_err(failure::Error::from)
                .map_err(|e| e.context("failed to describe spot instances for {}"))?;
            let instance_requests = result.spot_instance_requests;
            let any_open = instance_requests
                .iter()
                .flatten()
                .map(|it| (it, it.state.as_ref()))
                .any(|(sir, state)| {
                    state.is_some_and(|s| s == "open")
                        || (state.is_some_and(|s| s == "active") && sir.instance_id.is_none())
                });
            if !any_open {
                all_active = true;
                break instance_requests
                    .into_iter()
                    .flatten()
                    .filter_map(|r| {
                        if r.state.as_ref().is_some_and(|s| s == "active") {
                            let name = id_to_name.remove(
                                &r.spot_instance_request_id
                                    .expect("any spot instance request should have an id"),
                            );
                            id_to_name.insert(
                                r.instance_id.as_ref().expect("in above code we ensured that no instance_id will be considred as not open").clone()
                                ,name.expect(
                                    "the name is proveded by us and we expect to have value",
                                ),
                            );
                            Some(r.instance_id)
                        } else {
                            all_active = false;
                            None
                        }
                    })
                    .filter_map(|instance_id| instance_id)
                    .collect();
            } else {
                use std::{thread, time::Duration};
                thread::sleep(Duration::from_millis(200));
            }
        };

        //Stop spot requests
        let mut cancel = rusoto_ec2::CancelSpotInstanceRequestsRequest::default();
        cancel.spot_instance_request_ids = desc_req
            .spot_instance_request_ids
            .take()
            .expect("the describe spot instance request should have spot request ids by now");
        ec2.cancel_spot_instance_requests(cancel)
            .await
            .map_err(failure::Error::from)
            .map_err(|e| e.context("failed to cancel spot instance request"))?;

        // 2. wait for instances to come up

        let mut machines = HashMap::new();

        let mut desc_instance_req = rusoto_ec2::DescribeInstancesRequest::default();
        desc_instance_req.instance_ids = Some(instance_ids.clone());

        let mut all_ready = false;
        while !all_ready {
            all_ready = true;
            machines.clear();

            for reservation in ec2
                .describe_instances(desc_instance_req.clone())
                .await
                .map_err(failure::Error::from)
                .map_err(|e| e.context("failed to describe instances"))?
                .reservations
                .unwrap_or_else(Vec::new)
            {
                for instance in reservation.instances.unwrap_or_else(Vec::new) {
                    match instance {
                        rusoto_ec2::Instance {
                            instance_id: Some(instance_id),
                            instance_type: Some(instance_type),
                            public_dns_name: Some(public_dns),
                            private_ip_address: Some(private_ip),
                            public_ip_address: Some(public_ip),
                            ..
                        } => {
                            let name = id_to_name[&instance_id].clone();
                            machines.entry(name).or_insert_with(Vec::new).push(Machine {
                                ssh: None,
                                private_ip,
                                public_dns,
                                instance_type,
                                public_ip,
                            });
                        }
                        _ => {
                            all_ready = false;
                            continue;
                        }
                    }
                }
                if !all_ready {
                    continue;
                }
            }
        }

        // defer!{{
        //      let mut terminate_req = rusoto_ec2::TerminateInstancesRequest::default();
        // terminate_req.instance_ids = instance_ids;
        // tokio::task::spawn(async{
        // while let Err(e) = ec2.terminate_instances(terminate_req.clone()).await {
        //     let msg = format!("{}", e);
        //     if msg.contains("Pooled stream disconnected") || msg.contains("broken pip") {
        //         continue;
        //     } else {
        //         return Err::<(),failure::Error>(failure::Error::from(e)
        //             .context("failed to terminate instances")
        //             .into());
        //     }
        // }
        // Ok(())});

        // }}

        //TODO: ensure the number of machines are the same as been requested

        let mut errors = Vec::new();
        if all_active {
            for (name, machines) in &mut machines {
                let descriptor = &self.descriptors[name];
                let setup = &descriptor.0.setup;
                //TODO
                //Setup the machines in parallel (rayon)
                errors.par_extend(
                    machines
                        .par_iter_mut()
                        .map(|machine| -> Result<_, failure::Error> {
                            let addr = {
                                use std::net::{IpAddr, SocketAddr};

                                SocketAddr::new(
                                    IpAddr::from_str(&machine.public_ip)
                                        .context("the machine public ip address is not valid")?,
                                    22,
                                )
                            };
                            let ssh = ssh::Session::connect(addr, key_pair_file.path())
                                .map_err(failure::Error::from)
                                .map_err(|e| {
                                    e.context(format!(
                                        "the ssh connection failed for {} to {}",
                                        name, machine.public_dns
                                    ))
                                })?;
                            machine.ssh = Some(ssh);
                            setup(machine.ssh.as_mut().expect("the ssh has value"))
                                .map_err(failure::Error::from)
                                .map_err(|e| {
                                    e.context(format!(
                                        "setup procedure for {} on {} failed",
                                        name, machine.public_dns
                                    ))
                                })?;
                            Ok(())
                        })
                        .filter_map(Result::err),
                );
            }
        }

        //TODO : Create a SecurityGroup in AWS to allow my ip address do ssh, and fill out key_name
        //and security group of the launch_specification

        //   - once the instances are ready , run the setup closures :MachineSetup.setup
        // 3. make sure all setups are done and step 2 is done completly
        //
        // 4. Run the script closure

        if errors.is_empty() {
            script(machines)
                .map_err(failure::Error::from)
                .map_err(|e| e.context("main procedure failed"))?;
        }
        // 5. terminate all instances

        let mut terminate_req = rusoto_ec2::TerminateInstancesRequest::default();
        terminate_req.instance_ids = instance_ids;
        while let Err(e) = ec2.terminate_instances(terminate_req.clone()).await {
            let msg = format!("{}", e);
            if msg.contains("Pooled stream disconnected") || msg.contains("broken pip") {
                continue;
            } else {
                return Err(failure::Error::from(e)
                    .context("failed to terminate instances")
                    .into());
            }
        }

        //Clean up security group and key-pairs
        let req = rusoto_ec2::DeleteSecurityGroupRequest {
            group_id: Some(group_id),
            ..Default::default()
        };

        let mut retries = 0;

        loop {
            match ec2.delete_security_group(req.clone()).await {
                Ok(_) => break,
                Err(_) if retries < 5 => {
                    std::thread::sleep(std::time::Duration::from_secs(10 * retries + 1));
                    retries += 1;
                }
                Err(e) => {
                    return Err(failure::Error::from(e)
                        .context("failed to remove the security group")
                        .into());
                }
            }
        }
        let req = rusoto_ec2::DeleteKeyPairRequest {
            key_name: Some(key_name),
            ..Default::default()
        };
        ec2.delete_key_pair(req)
            .await
            .context("failed to remove the key-pair")?;

        errors.into_iter().next().map(|e| Err(e)).unwrap_or(Ok(()))
    }
}

//#[derive(Debug)]
pub struct Machine {
    ssh: Option<ssh::Session>,
    instance_type: String,
    pub private_ip: String,
    pub public_dns: String,
    pub public_ip: String,
}

impl Machine {
    pub fn run(&self, command: &str) -> Result<String, failure::Error> {
        if let Some(ssh) = &self.ssh {
            return ssh.cmd(command);
        }

        Err(failure::Context::from("the ssh for the machin is not initilized").into())
    }
}
