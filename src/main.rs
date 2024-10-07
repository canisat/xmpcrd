mod xm;
mod hw;

use xm::command::{send_command, XMCommand};
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

    match send_command(&mut *port, XMCommand::get_radio_id(), timeout) {
        Ok(serial_number_str) => {
            println!("Received valid XMRadioID: {}", serial_number_str);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
