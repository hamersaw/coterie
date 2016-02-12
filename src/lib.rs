#[macro_use] extern crate log;
extern crate protobuf;

pub mod dht;
pub mod message;

use std::net::{Ipv4Addr,SocketAddrV4};
use std::str::FromStr;

pub fn parse_socket_addr_v4(ip_address: &str, port: u16) -> Result<SocketAddrV4, String> {
    let ip = match Ipv4Addr::from_str(ip_address) {
        Ok(ip) => ip,
        Err(e) => return Err(format!("Error parsing ip address: {}", e)),
    };

    Ok(SocketAddrV4::new(ip, port))
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
