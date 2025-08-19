use std::io::Result;
use tun_tap;

fn main() -> Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    loop {
        let mut buff = [0u8; 1504];

        let len = nic.recv(&mut buff[..])?;

        let flag = i16::from_be_bytes([buff[0], buff[1]]);
        let prot = i16::from_be_bytes([buff[2], buff[3]]);
        if prot != 0x800 {
            continue; // not a ipv4 packet
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buff[4..len]) {
            Ok(p) => {
                let src = p.source_addr();
                let dst = p.destination_addr();
                let protocol = p.protocol().0;
                let payload_len = p
                    .payload_len()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                eprintln!(
                    "{} -> {} , protocol: {} , payload_len: {}",
                    src, dst, protocol, payload_len
                );
            }
            Err(_) => {
                //do nothing , some unrelated packets
            }
        }
    }
    Ok(())
}
