use crate::dns_error::DNSError;
use crate::server::Server;
use std::env;
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr};

pub mod dns_error;
pub mod dns_message;
pub mod server;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let Ok(follow_server) = parse_arg() else {
        println!("Usage: dns_server <follow_server_ip:port>");
        return Err(Box::new(DNSError::NoFollowServer));
    };
    let listen_ip = Ipv4Addr::new(0, 0, 0, 0);
    let port = 2053;
    let server = Server::new(listen_ip, port, follow_server)?;

    server.run()?;
    Ok(())
}

fn parse_arg() -> Result<SocketAddr, DNSError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args[1] != "--resolver" {
        return Err(DNSError::NoFollowServer);
    }
    args[2]
        .parse::<SocketAddr>()
        .map_err(|_| DNSError::FollowServerParseError)
}
