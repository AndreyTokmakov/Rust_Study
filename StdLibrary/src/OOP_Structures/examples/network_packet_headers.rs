
mod network_packet_headers
{
    struct IPv4Header(u8, u32, u32, u16, u8);

    fn parse_packet(packet: &[u8]) -> IPv4Header {
        // Simplified parsing logic
        IPv4Header(
            packet[0],                        // Version & header length
            u32::from_be_bytes([packet[1], packet[2], packet[3], packet[4]]), // Source IP
            u32::from_be_bytes([packet[5], packet[6], packet[7], packet[8]]), // Dest IP
            u16::from_be_bytes([packet[9], packet[10]]), // Length
            packet[11],                       // Protocol
        )
    }

    pub fn demo()
    {
        // Simulated packet data
        let raw_packet_data: [u8; 12] = [
            0x45, 0xc0, 0xa8, 0x01, 0x01, 0xc0,
            0xa8, 0x01, 0x02, 0x00, 0x50, 0x06
        ];

        let header: IPv4Header = parse_packet(&raw_packet_data);

        println!("Packet version: {}", header.0 >> 4);
        println!("From: {}.{}.{}.{}",
                 (header.1 >> 24) & 0xFF,
                 (header.1 >> 16) & 0xFF,
                 (header.1 >> 8) & 0xFF,
                 header.1 & 0xFF);
    }
}

pub fn test_all()
{
    network_packet_headers::demo();

    // Packet version: 4
    // From: 192.168.1.1
}