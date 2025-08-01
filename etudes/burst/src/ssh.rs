use failure::{self, ResultExt};
use ssh2;
use std::net::{self, TcpStream};

pub struct Session {
    ssh: ssh2::Session,
}

impl Session {
    pub(crate) fn connect<A: net::ToSocketAddrs>(
        addr: A,
        private_key_path: &Path,
    ) -> Result<Self, failure::Error> {
        let mut retries = 0;
        let tcp = loop {
            match TcpStream::connect(&addr) {
                Ok(tcp) => break tcp,
                Err(_) if retries < 6 => {
                    std::thread::sleep(std::time::Duration::from_secs(10 * retries + 1));
                    retries += 1
                }
                Err(e) => {
                    Err(failure::Error::from(e).context("Failed to stablish tcp connection"))?
                }
            }
        };

        let mut session = ssh2::Session::new().context("Failed to initilize a new ssh session")?;
        session.set_tcp_stream(tcp);
        session
            .handshake()
            .context("Failed to perform handshake on ssh session")?;

        //let private_key_path = Path::new("/home/meyar/.ssh/bonjour.pem");
        session
            .userauth_pubkey_file("ec2-user", None, private_key_path, None)
            .context("Failed to authenticate the ssh")?;

        Ok(Self { ssh: session })
    }

    pub fn cmd(&self, command: &str) -> Result<String, failure::Error> {
        use std::io::Read;
        let mut channel = self
            .ssh
            .channel_session()
            .map_err(failure::Error::from)
            .map_err(|e| {
                e.context(format!(
                    "failed to stablish a session channel for `{}`",
                    command
                ))
            })?;
        channel
            .exec(command)
            .context(format!("failed to execute the command `{}`", command))?;

        let mut s = String::new();
        channel
            .read_to_string(&mut s)
            .context(format!("failed to read the `{}` command result", command))?;
        channel.wait_close().context(format!(
            "failed to close the channel for `{}` command",
            command
        ))?;

        //TODO: ensure the channel exit status
        Ok(s)
    }
}

use std::ops::{Deref, DerefMut};
use std::path::Path;

impl Deref for Session {
    type Target = ssh2::Session;
    fn deref(&self) -> &Self::Target {
        &self.ssh
    }
}

impl DerefMut for Session {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ssh
    }
}
