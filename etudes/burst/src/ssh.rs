use ssh2;
use std::io;
use std::net::{self, TcpStream};

pub struct Session {
    ssh: ssh2::Session,
}

impl Session {
    pub(crate) fn connect<A: net::ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        let mut retries = 0;
        let tcp = loop {
            match TcpStream::connect(&addr) {
                Ok(tcp) => break tcp,
                Err(_) if retries < 3 => retries += 1,
                Err(e) => return Err(e),
            }
        };

        let mut session =
            ssh2::Session::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        session.set_tcp_stream(tcp);
        session
            .handshake()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let private_key_path = Path::new("/home/meyar/.ssh/bonjour.pem");
        session
            .userauth_pubkey_file("ec2-user", None, &private_key_path, None)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(Self { ssh: session })
    }

    pub fn cmd(&self, command: &str) -> Result<String, io::Error> {
        use io::Read;
        let mut channel = self.ssh.channel_session()?;
        channel.exec(command)?;
        let mut s = String::new();
        channel.read_to_string(&mut s)?;
        channel.wait_close()?;
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
