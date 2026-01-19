use crate::server::Server;
use std::env;
use std::net::Ipv4Addr;

pub mod dns_error;
pub mod dns_message;
pub mod server;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let follow_server;
    if args.len() == 3 {
        let Ok(ip) = args[2].parse::<Ipv4Addr>() else {
            panic!("Error: resolver ip address wrong format");
        };
        follow_server = Some(ip);
    } else {
        follow_server = None;
    }
    let listen_ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 2053;
    let server = Server::new(listen_ip, port, follow_server);

    server.run()?;
    Ok(())
}
