use burst::{BurstBuilder, Machine, MachineSetup};
use std::collections::HashMap;
use std::io::Read;
use tokio::runtime::Runtime;

fn main() {
    let mut builder = BurstBuilder::default();
    builder.add_setup(
        "server".to_string(),
        1,
        MachineSetup::new("t2.micro", "ami-083e865b97bdf1c1b", |ssh| {
            let mut channel = ssh.channel_session()?;
            channel.exec("ip addr show up")?;
            let mut s = String::new();
            channel.read_to_string(&mut s)?;
            println!("{}", s);
            channel.wait_close()?;
            println!("{}", channel.exit_status()?);
            Ok(())
            //            ssh.exec("sudo apt install cargo")
        }),
    );
    builder.add_setup(
        "client".to_string(),
        1,
        MachineSetup::new("t2.micro", "ami-083e865b97bdf1c1b", |ssh| {
            //          ssh.exec("sudo apt install cargo")
            Ok(())
        }),
    );

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        builder
            .run(|vms: HashMap<String, &[Machine]>| {
                //let server = &vms["server1"][0].private_ip;
                //let command = format!("ping {}", server);

                //vms["client"].for_each_parallel(|client| client.exec(&command));
                Ok(())
            })
            .await;
    });
}
