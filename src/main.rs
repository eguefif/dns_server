use crate::server::Server;
use crate::dns_error::DNSError;
use std::env;
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr};

pub mod dns_error;
pub mod dns_message;
pub mod server;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let follow_server = parse_arg()?;
    let listen_ip = Ipv4Addr::new(0, 0, 0, 0);
    let port = 2053;
    let server = Server::new(listen_ip, port, follow_server)?;

    server.run()?;
    Ok(())
}

fn parse_arg() -> Result<SocketAddr, DNSError> {
    let args: Vec<String> = env::args().collect();
    let follow_server;
    if args.len() == 3 {
        if args[2].contains(':') {
            let mut splits = args[2].split(':');
            let ip = splits.next().unwrap();
            let port = splits.next().unwrap();

            let Ok(ip) = ip.parse::<Ipv4Addr>() else {
                panic!("Error: resolver ip address wrong format");
            };
            let Ok(port) = port.parse::<u16>() else {
                panic!("Error: resolver port wrong format");
            };
            follow_server = std::net::SocketAddr::from((ip, port));
            println!("Follow server: {:?}", follow_server);
        } else {
            panic!("Error: resolver format should be <ip>:<port>.")
        }
    } else {
        return Err(DNSError::NoFollowServer);
    }
    Ok(follow_server)
}
