use rusoto_ec2::Ec2;
use ssh2::Session;
use std::{collections::HashMap, io::Read, net::TcpStream};

mod ssh;

extern crate rusoto;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_ec2;
extern crate ssh2;

struct Burst {}

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
    setup: Box<dyn Fn(&mut ssh::Session) -> std::io::Result<()>>,
}

impl MachineSetup {
    pub fn new<F>(instance_type: &str, ami: &str, setup: F) -> Self
    where
        F: Fn(&mut ssh::Session) -> std::io::Result<()> + 'static,
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

    pub async fn run<F>(&mut self, script: F)
    where
        F: FnMut(std::collections::HashMap<String, &[Machine]>) -> std::io::Result<()>,
    {
        //use rusoto_core::Region;
        //use rusoto_credential::DefaultCredentialsProvider;
        //let provider = DefaultCredentialsProvider::new().unwrap();

        let ec2 = rusoto_ec2::Ec2Client::new(rusoto_core::Region::UsEast1);
        // 1. issue spot requests
        //
        let mut id_to_name = HashMap::new();
        let mut spot_instance_request_ids = Vec::new();
        for (name, (setup, number)) in &self.descriptors {
            let launch = rusoto_ec2::RequestSpotLaunchSpecification {
                image_id: Some(setup.ami.clone()),
                instance_type: Some(setup.instance_type.clone()),
                security_groups: Some(vec!["bonjour".to_string()]),
                key_name: Some("bonjour".to_string()),
                ..Default::default()
            };

            let req = rusoto_ec2::RequestSpotInstancesRequest {
                launch_specification: Some(launch),
                //block_duration_minutes: Some(self.max_duration_time),
                instance_count: Some(*number),
                //instance_interruption_behavior: Some("stop".to_string()),
                ..Default::default()
            };
            let result = ec2.request_spot_instances(req.clone()).await.unwrap();
            let requests = result.spot_instance_requests.unwrap();
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
        let instance_ids: Vec<String> = loop {
            let result = ec2.describe_spot_instance_requests(desc_req.clone());
            let instance_requests = result.await.unwrap().spot_instance_requests;
            let any_open = instance_requests
                .iter()
                .flatten()
                .any(|it| it.state.as_ref().is_some_and(|s| s == "open"));
            if !any_open {
                break instance_requests
                    .into_iter()
                    .flatten()
                    .filter_map(|it| {
                        let name = id_to_name.remove(&it.spot_instance_request_id.unwrap());
                        id_to_name.insert(it.instance_id.as_ref().unwrap().clone(), name.unwrap());
                        it.instance_id
                    })
                    .collect();
            }
        };

        //Stop spot requests
        let mut cancel = rusoto_ec2::CancelSpotInstanceRequestsRequest::default();
        cancel.spot_instance_request_ids = desc_req.spot_instance_request_ids.take().unwrap();

        ec2.cancel_spot_instance_requests(cancel).await.unwrap();
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
                .unwrap()
                .reservations
                .unwrap()
            {
                for instance in reservation.instances.unwrap() {
                    match instance {
                        rusoto_ec2::Instance {
                            instance_id: Some(instance_id),
                            instance_type: Some(instance_type),
                            public_dns_name: Some(public_dns),
                            private_ip_address: Some(private_ip),
                            ..
                        } => {
                            let name = id_to_name[&instance_id].clone();
                            machines.entry(name).or_insert_with(Vec::new).push(Machine {
                                ssh: None,
                                private_ip,
                                public_dns,
                                instance_type,
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

        for (name, machines) in &mut machines {
            let descriptor = &self.descriptors[name];
            let setup = &descriptor.0.setup;
            for machine in machines {
                loop {
                    println!("try to run the setup script on {}", machine.public_dns);
                    match ssh::Session::connect(format!("{}:22", machine.public_dns)) {
                        Ok(session) => {
                            machine.ssh = Some(session);
                            if let Some(session) = machine.ssh.as_mut() {
                                setup(session).unwrap();
                                break;
                            } else {
                                std::thread::sleep(std::time::Duration::from_secs(5));
                                continue;
                            }
                        }
                        Err(e) => {
                            println!("{:#?}", e);
                            std::thread::sleep(std::time::Duration::from_secs(5));
                            continue;
                        }
                    }
                }
            }
        }

        //TODO : Create a SecurityGroup in AWS to allow my ip address do ssh, and fill out key_name
        //and security group of the launch_specification

        //   - once the instances are ready , run the setup closures :MachineSetup.setup
        // 3. make sure all setups are done and step 2 is done completly
        //
        //
        //
        //
        // 4. terminate all instances

        let mut terminate_req = rusoto_ec2::TerminateInstancesRequest::default();
        terminate_req.instance_ids = instance_ids;

        let _ = ec2.terminate_instances(terminate_req).await.unwrap();
    }
}

//#[derive(Debug)]
pub struct Machine {
    ssh: Option<ssh::Session>,
    instance_type: String,
    pub private_ip: String,
    pub public_dns: String,
}
