use std::net::{SocketAddr, TcpStream, IpAddr, Ipv4Addr};
use std::process;
use std::env;
use std::time::Duration;

const MIN_PORT_NUMBER : u16 = 0;
const MAX_PORT_NUMBER : u16 = 65535;
const HOST : IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

fn main() {
    let args: Vec<String> = env::args().collect();

    // mutable list to hold the open TCP ports
    let mut open_tcp_ports : Vec<u16> = Vec::with_capacity(MAX_PORT_NUMBER.into());

    if args.len() > 1 {
        // we specified a specific port, check that this is a valid port
        let number = &args[1];
        let specific_port: u16 = match number.parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                eprintln!("Error: port provided is not a number");
                process::exit(1);
            },
        };
        //specific_port = &args[1].parse::<u16>();
        if specific_port < MIN_PORT_NUMBER || specific_port > MAX_PORT_NUMBER {
            println!("Port number provided is out of range.");
            process::exit(1);
        }
        else {
            scan_port(&mut open_tcp_ports, specific_port);
            if open_tcp_ports.len() == 1 {
                println!("Port {} is open.", specific_port);
            }
            else {
                println!("Port {} is closed.", specific_port);
            }
        }
    }

    // if no specific port given:
    else {
        for port in MIN_PORT_NUMBER..=MAX_PORT_NUMBER {
            scan_port(&mut open_tcp_ports, port);
        }

        print_open_ports(&mut open_tcp_ports);
    }
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