use crate::packet_data::PacketData;

pub fn individual_display(num: u32, packet: &PacketData, incl_data: bool) {
    clear_display();
    print_header();
    print_packet(num, &packet, incl_data);
}

pub fn print_header() {
    println!(
        "{:<10} | {:<20} | {:<40} | {:<40} | {:<10} | {:<10}",
        "Num:", "Time:", "Source:", "Destination:", "Protocol:", "Length:"
    );
    println!("{}", "=".repeat(145));
}

pub fn print_packet(num: u32, packet: &PacketData, incl_data: bool) {
    println!(
        "{:<10} | {:<20} | {:<40} | {:<40} | {:<10} | {:<10}",
        num,
        packet.get_ts(),
        packet.source_ip,
        packet.destination_ip,
        packet.protocol,
        packet.length
    );

    if incl_data {
        println!("Data:");
        println!("{}", packet.data);
    }
}

pub fn print_help() {
    println!("WireCrab - A terminal utility for capturing the network packets and display them on screen.\n");
    println!("Usage:\n");
    println!("wirecrab <args>\n");
    println!("Arguments:");
    println!("\t-m -- display mode (0, 1, 2, 3) [REQUIRED]");
    println!("\t-f -- name of the output file [REQUIRED]");
    println!("\t-c -- number of packages to be captured\n");
    println!("For more info please read the project's README.");
}

fn clear_display() {
    print!("\x1B[2J\x1B[1;1H");
}
