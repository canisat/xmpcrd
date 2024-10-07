pub struct XMPacket {
    data_length: u16,
    pub data: Vec<u8>,
}

impl XMPacket {
    pub(crate) fn new(data: Vec<u8>) -> Self {
        let data_length = data.len() as u16;
        XMPacket { data_length, data }
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut command: Vec<u8> = vec![0x5A, 0xA5]; // Start bytes
        command.extend_from_slice(&self.data_length.to_be_bytes()); // Length in big-endian
        command.extend_from_slice(&self.data); // Command data
        command.extend_from_slice(&[0xED, 0xED]); // Footer
        command
    }

    // Helper function to validate the response format
    pub(crate) fn is_valid_response(response: &[u8]) -> bool {
        // Validate that the response is at least the minimum size for a valid packet (6 bytes)
        if response.len() < 6 {
            return false;
        }
    
        // Check the start of the response (5A A5)
        if response[0] != 0x5A || response[1] != 0xA5 {
            return false;
        }
    
        // Extract the length field (xx xx), which is in big-endian format
        let length = u16::from_be_bytes([response[2], response[3]]) as usize;
    
        // Ensure the total response length matches what is indicated in the length field + header + footer
        // Header (4 bytes) + Data (length bytes) + Footer (2 bytes)
        if response.len() != length + 6 {
            return false;
        }
    
        // If all checks pass, the response is valid
        true
    }
    
}
