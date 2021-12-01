use std::net::{SocketAddr, TcpStream, IpAddr, Ipv4Addr};
use std::process;
use std::time::Duration;

const MIN_PORT_NUMBER : u16 = 0;
const MAX_PORT_NUMBER : u16 = 65535;
const HOST : IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

pub fn port_scan(port: u16) -> String
{
    // mutable list to hold the open TCP ports
    let mut open_tcp_ports : Vec<u16> = Vec::with_capacity(MAX_PORT_NUMBER.into());

    if port > 0 {
        
        if port < MIN_PORT_NUMBER || port > MAX_PORT_NUMBER {
            println!("Port number provided is out of range.");
            process::exit(1);
        }
        else {
            scan_port(&mut open_tcp_ports, port);
            if open_tcp_ports.len() == 1 {
                let final_string = "Port ".to_owned() + &port.to_string() + &" is open.".to_owned();
                return final_string;
            }
            else {
                let final_string = "Port ".to_owned() + &port.to_string() + &" is closed.".to_owned();
                return final_string;
            }
        }
    }

    // if no specific port given, value of port should be less than 0
    else {
        for port in MIN_PORT_NUMBER..=MAX_PORT_NUMBER {
            scan_port(&mut open_tcp_ports, port);
        }

        let final_string = print_open_ports(&mut open_tcp_ports);
        return final_string;
    }
}

pub fn scan_port(open_tcp_ports: &mut Vec<u16>, port: u16) {
    // create new internet socket address, combining localhost ipaddr with port num
    let socket = SocketAddr::new(HOST, port);
    let timeout = Duration::new(2, 0); // two second timeout
    if let Ok(_stream) = TcpStream::connect_timeout(&socket, timeout) {
        // add this open port to the list
        open_tcp_ports.push(socket.port());
    }
    // else do nothing, this port is closed.
}

pub fn print_open_ports(open_tcp_ports: &mut Vec<u16>) -> String
{
    //println!("{:?}", open_tcp_ports);
    let mut final_string = "The following ports are open:\n".to_owned();

    for port in open_tcp_ports.iter() {
        final_string.push_str(&port.to_string());
        final_string.push_str(&"\n".to_owned());
    }

    return final_string;
}
