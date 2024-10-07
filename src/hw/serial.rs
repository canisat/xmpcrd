use std::io;
use std::time::Duration;
use serialport::{SerialPort, DataBits, FlowControl, Parity, StopBits};

pub fn xm_pcr_serial(port_name: &str, baud_rate: u32, timeout: Duration) -> Result<Box<dyn SerialPort>, io::Error> {

    let port: Result<Box<dyn SerialPort>, io::Error> = serialport::new(port_name, baud_rate)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(timeout)
        .open()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to open serial port: {}", e)));

    return port;
}