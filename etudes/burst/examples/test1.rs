use burst::{BurstBuilder, Machine, MachineSetup};
use std::collections::HashMap;
use tokio::runtime::Runtime;

fn main() {
    let mut builder = BurstBuilder::default();
    builder.use_term_logger();
    builder.add_setup(
        "server".to_string(),
        1,
        MachineSetup::new("t2.micro", "ami-083e865b97bdf1c1b", |ssh| {
            let result = ssh.cmd("cat etc/hostname")?;
            println!("ip addr: {}", result);

            Ok(())
        }),
    );
    builder.add_setup(
        "client".to_string(),
        8,
        MachineSetup::new("t2.micro", "ami-083e865b97bdf1c1b", |ssh| {
            let result = ssh.cmd("date")?;
            println!("date: {}", result);

            Ok(())
        }),
    );

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        builder
            .run(|vms: HashMap<String, Vec<Machine>>| {
                println!("server private ip: {}", vms["server"][0].private_ip);
                println!("client private ip: {}", vms["client"][0].private_ip);
                // let server = &vms["server"][0].private_ip;
                // let command = format!("ping {}", server);

                // for client in &vms["client"] {
                //     if let Ok(result) = client.run(&command) {
                //         println!("{}", result);
                //     }
                // }
                Ok(())
            })
            .await
            .unwrap();
    });
}
