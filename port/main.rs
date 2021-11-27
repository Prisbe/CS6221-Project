use std::net::{SocketAddr, TcpStream, IpAddr, Ipv4Addr};
use std::env;
use std::time::Duration;

const MIN_PORT_NUMBER : u16 = 0;
const MAX_PORT_NUMBER : u16 = 65535;
const HOST : IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

fn main() {
    let args: Vec<String> = env::args().collect();

    // mutable list to hold the open TCP ports
    let mut open_tcp_ports : Vec<u16> = Vec::with_capacity(MAX_PORT_NUMBER.into());

    for port in MIN_PORT_NUMBER..=MAX_PORT_NUMBER {
        scan_port(&mut open_tcp_ports, port);
    }

    print_open_ports(&mut open_tcp_ports);
}

fn scan_port(open_tcp_ports: &mut Vec<u16>, port: u16) {
    // create new internet socket address, combining localhost ipaddr with port num
    let socket = SocketAddr::new(HOST, port);
    let timeout = Duration::new(2, 0); // two second timeout
    if let Ok(_stream) = TcpStream::connect_timeout(&socket, timeout) {
        // add this open port to the list
        open_tcp_ports.push(socket.port());
    }
    // else do nothing, this port is closed.
}

fn print_open_ports(open_tcp_ports: &mut Vec<u16>) {
    println!("{:?}", open_tcp_ports);
}