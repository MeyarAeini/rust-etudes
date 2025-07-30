use ssh2;
use std::io;
use std::net::{self, TcpStream};

//#[derive(Debug)]
pub struct Session {
    ssh: ssh2::Session,
    //tcp: TcpStream,
}

impl Session {
    pub(crate) fn connect<A: net::ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        let tcp = TcpStream::connect(addr).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
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
        // session
        //   .userauth_agent("ec2-user")
        // .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(Self { ssh: session })
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
