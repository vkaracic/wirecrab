mod display;
mod modes;
mod utils;
mod packet_data;

use std::env;
use pcap::{Capture, Device};

fn main () {
    let args: Vec<String> = env::args().collect();
    for arg in args.iter() {
        if arg == "-h" || arg == "--help" {
            display::print_help();
            return;
        }
    }

    let mut limit = 100;

    let device = Device::lookup().unwrap();
    println!("Using device {}", device.name);
    let mut cap = Capture::from_device(device)
        .unwrap()
        .open()
        .unwrap();

    let filename = match utils::get_filename() {
        Err(err) => panic!(err),
        Ok(filename) => filename
    };

    let mode = match utils::get_mode() {
        Err(err) => panic!(err),
        Ok(mode) => mode
    };

    match utils::get_count() {
        Err(_) => (),
        Ok(count) => {
            limit = count;
        }
    };

    let mut savefile = cap.savefile(format!("{}.pcap", filename)).unwrap();

    display::print_header();

    let mut i: u32 = 1;
    while let Ok(packet) = cap.next() {
        let packet_data = packet_data::PacketData::new(&packet);

        match mode {
            modes::Modes::IndividualPrint => display::individual_display(i, &packet_data, false),
            modes::Modes::Default => display::print_packet(i, &packet_data, false),
            modes::Modes::IndividualPrintWithData => display::individual_display(i, &packet_data, true),
            modes::Modes::DefaultWithData => display::print_packet(i, &packet_data, true),
        }

        savefile.write(&packet);

        i+=1;
        if i > limit {
            break;
        }
    }
}
