use std::io;
use std::time::Duration;
use std::thread::sleep;
use serialport::SerialPort;
use crate::xm::packet::XMPacket;
use crate::xm::radio_id::XMRadioID;

pub struct XMCommand;

impl XMCommand {
    #[allow(dead_code)]
    pub fn power_on() -> XMPacket {
        XMPacket::new(vec![0x00, 0x10, 0x10, 0x10, 0x01])
    }
    #[allow(dead_code)]
    pub fn power_off() -> XMPacket {
        XMPacket::new(vec![0x01, 0x00])
    }
    #[allow(dead_code)]
    pub fn select_channel(channel: u8) -> XMPacket {
        XMPacket::new(vec![0x10, 0x02, channel, 0x00, 0x00, 0x01])
    }
    #[allow(dead_code)]
    pub fn get_radio_id() -> XMPacket {
        XMPacket::new(vec![0x31])
    }
}

pub fn send_command(port: &mut dyn SerialPort, command: XMPacket, timeout: Duration, expect_response: bool) -> Result<Option<String>, io::Error> {
    // Send the command
    port.write_all(&command.to_bytes())?;
    port.flush()?;

    if (!expect_response) {
        return Ok(None); // Return immediately if no response is expected
    }

    loop {
        // Read the response from the serial port
        let mut response: Vec<u8> = vec![0; 1024]; // Buffer for response
        match port.read(&mut response) {
            Ok(n) => {
                response.truncate(n);

                // Validate the response as an XMPacket
                if XMPacket::is_valid_response(&response) {
                    let packet = XMPacket::new(response);

                    // Parse the XMPacket into an XMRadioID
                    match XMRadioID::from_packet(&packet) {
                        Ok(radio_id) => {
                            let serial_number_bytes = radio_id.get_radio_id();
                            let serial_number_str = String::from_utf8_lossy(serial_number_bytes).to_string();
                            return Ok(Some(serial_number_str)); // Return the serial number string
                        },
                        Err(e) => {
                            eprintln!("Error parsing XMRadioID: {}", e);
                        }
                    }
                } else {
                    eprintln!("Received invalid XMPacket");
                }
            },
            Err(e) => {
                eprintln!("Error reading from serial port: {}", e);
            }
        }

        // Wait for the timeout period before retrying
        sleep(timeout);
    }
}
