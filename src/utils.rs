use core::fmt::Write;
use std::env;
use crate::modes;

pub fn get_filename() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    let error_missing = "Error: missing filename value".to_string();

    for (i, arg) in args.iter().enumerate() {
        if arg == "-f" {
           let next: usize = i + 1;
           match args.iter().nth(next) {
               None => return Err(error_missing),
               Some(value) => {
                   let filename = value.to_string();
                   return Ok(filename);
               }
           }
        }
    }

    return Err(error_missing);
}

pub fn get_count() -> Result<u32, String> {
    let args: Vec<String> = env::args().collect();
    let error_missing = "Error: missing count value".to_string();

    for (i, arg) in args.iter().enumerate() {
        if arg == "-c" {
           let next: usize = i + 1;
           match args.iter().nth(next) {
               None => return Err(error_missing),
               Some(c) => {
                   let count: u32 = c
                       .to_string()
                       .trim()
                       .parse()
                       .expect("Count has to be a number");
                   return Ok(count);
               }
           }
        }
    }

    return Err(error_missing);
}

pub fn get_mode() -> Result<&'static modes::Modes, String> {
    let args: Vec<String> = env::args().collect();
    let error_missing = "Error: missing mode value".to_string();
    let modes = modes::get_modes_map();

    for (i, arg) in args.iter().enumerate() {
        if arg == "-m" {
           let next: usize = i + 1;
           match args.iter().nth(next) {
               None => return Err(error_missing),
               Some(value) => {
                   let mode: u8 = value.to_string().trim().parse()
                       .expect("Mode has to be a number");

                   match modes.get(&mode) {
                       None => return Ok(&modes::Modes::Default),
                       Some(m) => return Ok(m)
                   }
               }
           }
        }
    }

    return Err(error_missing);
}

pub fn packet_data_to_hex(packet_data: &[u8]) -> String {
   let mut s = String::with_capacity(2 * packet_data.len());
   for byte in packet_data {
       write!(s, "{:02X} ", byte);
   }
   return s;
}
