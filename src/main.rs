mod xm;
mod hw;

use text_io::scan;
use xm::command::{XMCommand, send_command};
use hw::serial::xm_pcr_serial;
use std::time::Duration;
use std::io;

fn main() -> Result<(), io::Error> {
    let port_name: &str = "/dev/ttyUSB0"; // Adjust as per your system
    let baud_rate: u32 = 9600; // Adjust baud rate as needed
    let timeout: Duration = Duration::from_millis(500); // Short timeout

    let mut port = match xm_pcr_serial(port_name, baud_rate, timeout) {
        Ok(p) => p,  // Successfully got the serial port
        Err(e) => {
            eprintln!("Error opening serial port: {}", e);
            return Err(e); // Return the error if we can't open the port
        }
    };

    println!("Select the command you want to send: \n");
    println!("1. Power Radio On");
    println!("2. Power Radio Off");
    println!("3. Select Channel");
    println!("4. Get Channel Info");
    println!("5. Get Radio ID");
    println!("0. Exit");

    let a: i32;
    scan!("{}", a);

    match a {
        1 => {
            match send_command(&mut *port, XMCommand::power_on(), timeout, false) {
                Ok(_) => {
                    println!("Power on command sent.");
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        },
        2 => {
            match send_command(&mut *port, XMCommand::power_off(), timeout, false) {
                Ok(_) => {
                    println!("Power off command sent.");
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        },
        3 => {
            println!("Enter the channel number (0-255): ");
            let channel: u8;
            scan!("{}", channel);

            match send_command(&mut *port, XMCommand::select_channel(channel), timeout, false) {
                Ok(_) => {
                    println!("Channel selection command sent.");
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        },
        4 => {
            println!("Enter the channel number (0-255): ");
            let channel: u8;
            scan!("{}", channel);

            match send_command(&mut *port, XMCommand::get_channel_info(channel), timeout, true) {
            Ok(Some(response)) => {
                println!("Received valid XM packet: {}", response);
            },
            Ok(None) => {
                println!("No response expected.");
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
            }
        }
        5 => {
            match send_command(&mut *port, XMCommand::get_radio_id(), timeout, true) {
                Ok(Some(serial_number_str)) => {
                    println!("Received valid XMRadioID: {}", serial_number_str);
                },
                Ok(None) => {
                    println!("No response expected.");
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        },
        0 => {
            return Ok(());
        },
        _ => {
            eprintln!("Invalid command.");
            return Ok(());
        }
    }

    Ok(())
}
