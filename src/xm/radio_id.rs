use super::packet::XMPacket;

pub struct XMRadioID {
    radio_id: Vec<u8>,
}

impl XMRadioID {
    pub(crate) fn from_packet(packet: &XMPacket) -> Result<Self, &'static str> {
        let packet_bytes = packet.to_bytes();
        
        // Validate the packet
        if !XMPacket::is_valid_response(&packet_bytes) {
            return Err("Invalid packet");
        }

        // Extract the serial number (12 bytes starting from the 8th byte)
        if packet_bytes.len() < 20 {
            return Err("Packet too short to contain an XM Radio ID");
        }

        let radio_id = packet_bytes[8..20].to_vec();
        Ok(XMRadioID { radio_id })
    }

    pub fn get_radio_id(&self) -> &Vec<u8> {
        &self.radio_id
    }
}