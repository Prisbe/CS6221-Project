use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};
use pcap::Device;

/**
    Brian Prisbe is the author of this document.

    This function helps for quick conversion to a Hex string
*/
pub fn to_hex (value: u8) -> String {
    format!("{:02X}", value)
}
/**
    This function helps format the time the packet was captured
*/
fn get_time(packet: &pcap::Packet) -> String{
    let d = UNIX_EPOCH + Duration::from_secs(packet.header.ts.tv_sec as u64);
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%H:%M:%S").to_string();
    return timestamp_str;
}
/**
    This function helps format/parse the MAC Address and IP Proto
*/
fn ethernetFrame(packet: &pcap::Packet) -> String{
    let mut final_string = "MAC_Source:".to_owned() + &to_hex(packet.data[0]).to_string();
    for i in 1..6
    {
        let x = ":".to_owned() + &to_hex(packet.data[i]).to_string();
        final_string.push_str(&x);

    }
    let temp = " MAC_Destination";
    final_string.push_str(temp);
    for i in 6..12
    {
        let x = ":".to_owned() + &to_hex(packet.data[i]).to_string();
        final_string.push_str(&x);
    }
    let proto = " Proto:IPv4";
    final_string.push_str(&proto);

    return final_string

}
/**
    This function helps format/parse the IP Header that comes after the MAC Address/IP Proto
*/
fn ip_header(packet: &pcap::Packet) -> String {
    let version_header_length = packet.data[14]; //First byte of IP Header
    let version = version_header_length >> 4;
    let header_length = version_header_length & 0x0F;

    let mut final_string = "Header_Length:".to_owned() + &header_length.to_string();
    let version_string = " Version:".to_owned() + &version.to_string();
    final_string.push_str(&version_string);

    let ttl = packet.data[22];
    let ttl_string = " TTL:".to_owned() + &ttl.to_string() + "\n\t\t- ";

    let proto = packet.data[23];
    let proto_string = "IP_Protocol:".to_owned() + &proto.to_string();

    final_string.push_str(&ttl_string);
    final_string.push_str(&proto_string);

    let mut src_address = " Source_Address:".to_owned() + &packet.data[26].to_string();
    for i in 27..30
    {
        let x = ".".to_owned() + &packet.data[i].to_string();
        src_address.push_str(&x);
    }
    final_string.push_str(&src_address);

    let mut dest_address = " Destination_Address:".to_owned() + &packet.data[30].to_string();
    for i in 31..34
    {
        let x = ".".to_owned() + &packet.data[i].to_string();
        dest_address.push_str(&x);
    }
    final_string.push_str(&dest_address);

    return final_string;
}
/**
    This function helps format/parse the TCP Header that comes after IP Header
*/
fn tcp_header(packet: &pcap::Packet) -> String{
    let src_port = ((packet.data[34] as u32) << 8) | packet.data[35] as u32;
    let dest_port = ((packet.data[36] as u32) << 8) | packet.data[37] as u32;
    let src_port_string = "Source_Port:".to_owned() + &src_port.to_string();
    let dest_port_string = " Destination_Port:".to_owned() + &dest_port.to_string() + "\n\t\t-";
    let mut sequence:u32 = packet.data[38] as u32;
    for i in 39..42
    {
        sequence = (sequence << 8) | packet.data[i] as u32;
    }
    let sequence_string = " Sequence:".to_owned() + &sequence.to_string();

    let mut ackno : u32 = packet.data[42] as u32;

    for i in 42..45
    {
        ackno = (ackno << 8) | packet.data[i] as u32;
    }
    let ackno_string = " Acknowledgment:".to_owned() + &ackno.to_string() + "\n\t- Flags: \n\t\t- ";

    let mut flags: u16 = packet.data[45] as u16;
    flags = (flags << 8) | packet.data[46] as u16;

    let flag_urg_string = ((flags & 32) >> 5).to_string();
    let flag_ack_string = ((flags & 16) >> 4).to_string();
    let flag_psh_string = ((flags & 8) >> 3).to_string();
    let flag_rst_string = ((flags & 4) >>2).to_string();
    let flag_syn_string = ((flags & 2) >> 1).to_string();
    let flag_fin_string = (flags & 1).to_string();

    let mut flag_string = "URG: ".to_owned() + &flag_urg_string + ", ACK: "
        + &flag_ack_string + ", PSH: " + &flag_psh_string + ", RST: "
            + &flag_rst_string + ", SYN: " + &flag_syn_string + ", FIN: " + &flag_fin_string;

    let mut final_string = src_port_string;
    final_string.push_str(&dest_port_string);
    final_string.push_str(&sequence_string);
    final_string.push_str(&ackno_string);
    final_string.push_str(&flag_string);
    return final_string;
}
/**
    This function helps format/parse the Payload data that comes after TCP Header
*/
fn format_payload(packet: &pcap::Packet) -> String{
    let payload_size = packet.data.len() - 46; //Print all bytes not used
    let lines = payload_size / 15; //Amount of lines needed
    let newline_tab = "\n\t\t".to_owned();
    let space = " ".to_owned();
    //let extra: u32 = payload_size % 15; //Extra bytes left over
    let mut final_string = "\t\t".to_owned();


    if payload_size < 15 //Will Print 15 bytes per line
    {
        for i in 47..packet.data.len()
        {
            final_string.push_str(&to_hex(packet.data[i]).to_owned());
            final_string.push_str(&space);

        }
    }
    else
    {
            let mut i = 48;

            for j in 0..lines-1
            {
                for k in i..i+15
                {
                    final_string.push_str(&to_hex(packet.data[k]).to_owned());
                    final_string.push_str(&space);
                }
                final_string.push_str(&newline_tab);
                i = i + 15;
            }
    }

    return final_string;
}


/**
    This is the main function that the GUI will call. It will pass in an int to specify the
    the amount of packets that we want to grab
*/
pub fn get_n_packets() -> String
{
    let mut cap = Device::lookup().unwrap().open().unwrap(); //Grabs etho0

    let mut i: f32 = 0.0;
    while let Ok(packet) = cap.next(){
        if packet.data[23] == 6
        {
             let final_string = "ETHERNET FRAME @ TIME:".to_owned() + &get_time(&packet) + "\n\t- " + &ethernetFrame(&packet)
            + " \n\t- IPv4 Packet:\n\t\t- " + &ip_header(&packet) + "\n\t- TCP Segment:\n\t\t- " + &tcp_header(&packet)
            + "\n\t- Data: \n" + &format_payload(&packet) + "\n";
             return final_string.to_string(); //TODO We can send this string to the GUI
        }
    }
    return "error".to_string();
}

