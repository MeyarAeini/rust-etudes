//!`Burst` was the very first name which the creator of this crate during the live stream coding
//!used. Now you can find the original crate by searching for `tsunami` crate. I learned many
//!things from the creator of this crate , `Jon Gjengset`, and I will leanr many more other things
//!form him.
//!
//!This crate is my coding by following his steps on the original crate. This is only for leaning
//!purposes.
//!
//!```rust, no_run
//! # use burst::{BurstBuilder, Machine, MachineSetup};
//! # use std::collections::HashMap;
//! # use tokio::runtime::Runtime;
//!
//! # let mut builder = BurstBuilder::default();
//! # let rt = Runtime::new().unwrap();
//! # rt.block_on(
//! async {
//!        builder
//!            .run(|vms: HashMap<String, Vec<Machine>>| {
//!                println!("server private ip: {}", vms["server"][0].private_ip);
//!                println!("client private ip: {}", vms["client"][0].private_ip);
//!
//!                Ok(())
//!            })
//!            .await
//!            .unwrap();
//!    }
//! # );
#![deny(missing_docs)]

use failure::ResultExt;
use rayon::prelude::*;
use rusoto_ec2::Ec2;
use slog::Discard;
use slog::Logger;
use std::collections::HashMap;
use std::str::FromStr;
use tokio::time;
#[macro_use]
extern crate slog;
use slog_term;

mod ssh;

extern crate failure;
extern crate failure_derive;
extern crate rusoto;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_ec2;
extern crate scopeguard;
extern crate ssh2;

/// The top level burst builder, which allows you to configure the environement variables and
/// machine descriptiors with specifying how many of a set of machine you need. It also allows you
/// specify the maximum duration that you need the machines for your benchmark. This builder allows
/// you specify the type of the logger that you need.
///
/// ```rust
///  # use burst::{BurstBuilder, MachineSetup};
/// let mut builder = BurstBuilder::default();
/// builder.use_term_logger();
/// builder.add_setup(
///     "server".to_string(),
///     1,
///     MachineSetup::new("t2.micro", "ami-083e865b97bdf1c1b", |ssh| {
///         let result = ssh.cmd("cat etc/hostname")?;
///         println!("ip addr: {}", result);

///         Ok(())
///     }),
/// );
/// ```
pub struct BurstBuilder {
    descriptors: std::collections::HashMap<String, (MachineSetup, i64)>,
    max_duration_time: i64,
    logger: Logger,
}

impl Default for BurstBuilder {
    fn default() -> Self {
        Self {
            descriptors: Default::default(),
            max_duration_time: 60,
            logger: Logger::root(Discard, o!()),
        }
    }
}

///Allows you define a template of AWS instance type and ami and the setup function to be ran for
///spawning aws ec2 instances.
///
///Note that only instances which are TODO
///
pub struct MachineSetup {
    instance_type: String,
    ami: String,
    setup: Box<dyn Fn(&mut ssh::Session) -> Result<(), failure::Error> + Sync>,
}

impl MachineSetup {
    ///Creates  new AWS spot instance machin setup template
    ///```rust
    /// # use burst::MachineSetup;
    /// MachineSetup::new("t2.micro", "ami-083e865b97bdf1c1b", |ssh| {
    ///        let result = ssh.cmd("date")?;
    ///        println!("date: {}", result);
    ///
    ///          Ok(())
    ///   });
    ///```
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
    ///Add a new set of AWS machine setup configuration
    pub fn add_setup(&mut self, name: String, number: i64, setup: MachineSetup) {
        self.descriptors.insert(name, (setup, number));
    }

    ///Set the max duration hour for spawning the aws ec2 spot instances
    pub fn set_max_duration_hour(&mut self, hour: u8) {
        self.max_duration_time = hour as i64 * 60;
    }

    ///Assign a custom logger to the burst builder
    pub fn use_logger(&mut self, logger: Logger) {
        self.logger = logger;
    }

    ///Use the terminal logger as burst builder logger
    pub fn use_term_logger(&mut self) {
        use slog::Drain;
        use std::sync::Mutex;

        let decorator = slog_term::TermDecorator::new().build();
        let drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).fuse();

        self.logger = Logger::root(drain, o!());
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

    ///Run the main burst routine, and return erros in case of any error.
    pub async fn run<F>(&mut self, mut script: F) -> Result<(), failure::Error>
    where
        F: FnMut(std::collections::HashMap<String, Vec<Machine>>) -> Result<(), failure::Error>,
    {
        debug!(self.logger, "connecting to ec2");
        //use scopeguard::ScopeGuard;
        //Creates a client backed by the default tokio event loop.
        //The client will use the default credentials provider and tls client.
        let ec2 = rusoto_ec2::Ec2Client::new(rusoto_core::Region::UsEast1);
        debug!(self.logger, "connected to ec2");

        //Create a security group
        let security_group_name = format!("burst_sg_{}", Self::rand_string(10));
        trace!(self.logger,"Creating a secutiry group";"group_name"=>&security_group_name);
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

        trace!(self.logger, "security group created";"group_id"=>&group_id.clone());

        trace!(self.logger, "adding ip permissions to the security group");
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
        trace!(self.logger,"creating a key-pair";"key_name"=>&key_name);
        let key_pair_req = rusoto_ec2::CreateKeyPairRequest {
            key_name: key_name.clone(),
            ..Default::default()
        };

        let key_pair = ec2
            .create_key_pair(key_pair_req)
            .await
            .context("failed to generate the ec2 key-pairs")?;

        trace!(self.logger,"key-pair generated"; "fingerprint"=>key_pair.key_fingerprint);

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

        trace!(self.logger,"key pair private key stored into disk";"path"=>?key_pair_file.path());
        //prefixing ? prints the
        //display version of the
        //value

        // 1. issue spot requests
        let mut id_to_name = HashMap::new();
        let mut spot_instance_request_ids = Vec::new();
        debug!(self.logger, "issuing the spot requests");
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
            trace!(self.logger, "issuing spot request for {}",name; "number"=>number);
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
                        trace!(self.logger,"spot request issued for {}",name; "spot_instance_request_id"=>it.clone());
                        id_to_name.insert(it.clone(), name.clone());
                        it
                    }),
            );
        }
        debug!(self.logger, "describe the spot requests");
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
            trace!(
                self.logger,
                "Checking the status of each spot instance request"
            );
            let any_open = instance_requests
                .iter()
                .flatten()
                .map(|it| (it, it.state.as_ref()))
                .any(|(sir, state)| {
                    if state.is_some_and(|s| s == "open")
                        || (state.is_some_and(|s| s == "active") && sir.instance_id.is_none()) {
                            true
                    }
                    else {
                        trace!(self.logger, "spot instance request not yet ready";"state"=>state,"request"=>?sir);
                        false
                    }
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
                trace!(
                    self.logger,
                    "some spot instance requets are not ready yet, trying again ..."
                );
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
                            trace!(self.logger, "instance is ready"; "set"=>&name,"ip"=>&public_ip);
                            machines.entry(name).or_insert_with(Vec::new).push(Machine {
                                ssh: None,
                                private_ip,
                                public_dns,
                                _instance_type: instance_type,
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
                            debug!(self.logger,"setting up the instance for {}",&name;"ip"=>&machine.public_ip);
                            setup(machine.ssh.as_mut().expect("the ssh has value"))
                                .map_err(failure::Error::from)
                                .map_err(|e| {
                                    e.context(format!(
                                        "setup procedure for {} on {} failed",
                                        name, machine.public_dns
                                    ))
                                })?;
                            trace!(self.logger,"finish setting up for {}",&name;"ip"=>&machine.public_ip);
                            Ok(())
                        })
                        .filter_map(Result::err),
                );
            }
        }

        if errors.is_empty() {
            info!(self.logger, "running the burst");
            let start = time::Instant::now();
            script(machines)
                .context("main procedure failed")
                .map_err(|e| {
                    crit!(self.logger,"Error happend during runing the main procedure";"error"=>?e);
                    e
                })?;
            info!(self.logger,"the burst run it finished";"took"=>?start.elapsed());
        }
        // 5. terminate all instances

        let mut terminate_req = rusoto_ec2::TerminateInstancesRequest::default();
        terminate_req.instance_ids = instance_ids;
        while let Err(e) = ec2.terminate_instances(terminate_req.clone()).await {
            let msg = format!("{}", e);
            if msg.contains("Pooled stream disconnected") || msg.contains("broken pip") {
                trace!(self.logger, "retrying the termination instance requests";"message"=>?msg);
                continue;
            } else {
                return Err(failure::Error::from(e)
                    .context("failed to terminate instances")
                    .into());
            }
        }

        debug!(self.logger, "cleaning up the security groups and key-pairs");

        //Clean up security group and key-pairs
        let req = rusoto_ec2::DeleteSecurityGroupRequest {
            group_id: Some(group_id.clone()),
            ..Default::default()
        };

        let mut retries = 0;

        loop {
            match ec2.delete_security_group(req.clone()).await {
                Ok(_) => break,
                Err(e) if retries < 5 => {
                    debug!(self.logger,"deleting the secuity group failed, retrying";"error"=>?e);
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
        debug!(self.logger, "deleting the security group finished";"group_id"=>group_id);
        let req = rusoto_ec2::DeleteKeyPairRequest {
            key_name: Some(key_name.clone()),
            ..Default::default()
        };
        debug!(self.logger, "deleting the key-pair";"key_name"=>&key_name);
        ec2.delete_key_pair(req)
            .await
            .context("failed to remove the key-pair")?;

        info!(self.logger, "burst done!");
        errors.into_iter().next().map(|e| Err(e)).unwrap_or(Ok(()))
    }
}

///A handle to access the ec2 instance vlaues or configuations such as public ip, host name or
///private ip
//#[derive(Debug)]
pub struct Machine {
    ssh: Option<ssh::Session>,
    _instance_type: String,
    ///provides the private ip of the ec2 istance.
    pub private_ip: String,
    ///provides the public host name of the ec2 instance.
    pub public_dns: String,
    ///provide tge public ip of the ec2 instance
    pub public_ip: String,
}

impl Machine {
    ///run a procedure on an ec2 instance machine handler
    pub fn run(&self, command: &str) -> Result<String, failure::Error> {
        if let Some(ssh) = &self.ssh {
            return ssh.cmd(command);
        }

        Err(failure::Context::from("the ssh for the machin is not initilized").into())
    }
}
