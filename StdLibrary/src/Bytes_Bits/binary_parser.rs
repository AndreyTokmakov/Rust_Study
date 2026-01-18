


mod network_packet_parser
{
    use std::convert::TryInto;
    use std::fmt;

    #[derive(Debug)]
    pub enum ParseError
    {
        UnexpectedEof,
        InvalidEtherType(u16),
        InvalidIpVersion(u8),
    }

    pub struct BinaryReader<'a>
    {
        buf: &'a [u8],
        pos: usize,
    }

    impl<'a> BinaryReader<'a>
    {
        pub fn new(buf: &'a [u8]) -> Self {
            Self { buf, pos: 0, }
        }

        fn remaining(&self) -> usize {
            self.buf.len() - self.pos
        }

        pub fn read_u8(&mut self) -> Result<u8, ParseError>
        {
            if self.remaining() < 1 {
                return Err(ParseError::UnexpectedEof);
            }

            let v: u8 = self.buf[self.pos];
            self.pos += 1;
            Ok(v)
        }

        pub fn read_u16_be(&mut self) -> Result<u16, ParseError>
        {
            if self.remaining() < 2 {
                return Err(ParseError::UnexpectedEof);
            }

            let bytes: [u8; 2] = self.buf[self.pos..self.pos + 2]
                .try_into().unwrap();

            self.pos += 2;
            Ok(u16::from_be_bytes(bytes))
        }

        pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], ParseError>
        {
            if self.remaining() < len {
                return Err(ParseError::UnexpectedEof);
            }

            let slice = &self.buf[self.pos..self.pos + len];
            self.pos += len;

            Ok(slice)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EtherType
    {
        Ipv4,
        Arp,
        Ipv6,
        Unknown(u16),
    }

    impl EtherType
    {
        pub fn from_u16(value: u16) -> Self
        {
            match value
            {
                0x0800 => Self::Ipv4,
                0x0806 => Self::Arp,
                0x86DD => Self::Ipv6,
                other => Self::Unknown(other),
            }
        }

        pub fn raw(self) -> u16
        {
            match self
            {
                Self::Ipv4 => 0x0800,
                Self::Arp => 0x0806,
                Self::Ipv6 => 0x86DD,
                Self::Unknown(v) => v,
            }
        }
    }

    impl fmt::Display for EtherType
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            match self
            {
                Self::Ipv4 => write!(f, "IPv4 (0x0800)"),
                Self::Arp => write!(f, "ARP (0x0806)"),
                Self::Ipv6 => write!(f, "IPv6 (0x86dd)"),
                Self::Unknown(v) => write!(f, "Unknown (0x{:04x})", v),
            }
        }
    }

    #[derive(Debug)]
    pub struct EthernetHeader
    {
        pub dst_mac: [u8; 6],
        pub src_mac: [u8; 6],
        pub ethertype: EtherType,
    }

    #[derive(Debug)]
    pub struct Ipv4Header
    {
        pub ihl: u8,
        pub total_length: u16,
        pub protocol: u8,
        pub src_ip: [u8; 4],
        pub dst_ip: [u8; 4],
    }

    #[derive(Debug)]
    pub struct Packet
    {
        pub ethernet: EthernetHeader,
        pub ipv4: Ipv4Header,
        pub payload: Vec<u8>,
    }

    struct Mac<'a>(&'a [u8; 6]);
    struct Ipv4<'a>(&'a [u8; 4]);

    impl<'a> fmt::Display for Mac<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
            )
        }
    }

    impl<'a> fmt::Display for Ipv4<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3])
        }
    }

    impl<'a> fmt::Display for EthernetHeader {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "Ethernet:")?;
            writeln!(f, "  Src MAC : {}", Mac(&self.src_mac))?;
            writeln!(f, "  Dst MAC : {}", Mac(&self.dst_mac))?;
            writeln!(f, "  Type    : {}", &self.ethertype)
        }
    }

    impl<'a> fmt::Display for Ipv4Header {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "IPv4")?;
            writeln!(f, "  Src IP  : {}", Ipv4(&self.src_ip))?;
            writeln!(f, "  Dst IP  : {}", Ipv4(&self.dst_ip))?;
            writeln!(f, "  Proto   : {}", protocol_name(self.protocol))?;
            writeln!(f, "  Length  : {}", self.total_length)
        }
    }

    impl<'a> fmt::Display for Packet
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
            writeln!(f, "================ PACKET ================")?;
            writeln!(f, "{}", &self.ethernet)?;
            writeln!(f, "{}", &self.ipv4)?;
            writeln!(f, "Payload ({} bytes)", self.payload.len())?;
            writeln!(f, "  {:02x?}", self.payload)?;
            writeln!(f, "========================================")
        }
    }

    fn protocol_name(proto: u8) -> &'static str
    {
        match proto {
            1 => "ICMP",
            6 => "TCP",
            17 => "UDP",
            _ => "UNKNOWN",
        }
    }

    fn parse_ethernet(reader: &mut BinaryReader) -> Result<EthernetHeader, ParseError>
    {
        let dst_mac: [u8; 6] = reader.read_bytes(6)?.try_into().unwrap();
        let src_mac: [u8; 6] = reader.read_bytes(6)?.try_into().unwrap();
        let raw_type: u16 = reader.read_u16_be()?;

        Ok(EthernetHeader {
            dst_mac,
            src_mac,
            ethertype: EtherType::from_u16(raw_type),
        })
    }

    fn parse_ipv4(reader: &mut BinaryReader) -> Result<Ipv4Header, ParseError>
    {
        let version_ihl: u8 = reader.read_u8()?;
        let version: u8 = version_ihl >> 4;
        let ihl: u8 = version_ihl & 0x0F;

        if version != 4 {
            return Err(ParseError::InvalidIpVersion(version));
        }

        reader.read_u8()?; // DSCP + ECN
        let total_length: u16 = reader.read_u16_be()?;

        reader.read_u16_be()?; // identification
        reader.read_u16_be()?; // flags + fragment offset

        reader.read_u8()?; // TTL
        let protocol: u8 = reader.read_u8()?;
        reader.read_u16_be()?; // header checksum

        let src_ip: [u8; 4] = reader.read_bytes(4)?.try_into().unwrap();
        let dst_ip: [u8; 4] = reader.read_bytes(4)?.try_into().unwrap();

        let header_len: usize = (ihl as usize) * 4;
        let parsed_len: usize = 20;

        if header_len > parsed_len
        {
            let options_len = header_len - parsed_len;
            reader.read_bytes(options_len)?;
        }

        Ok(Ipv4Header { ihl,
                        total_length,
                        protocol,
                        src_ip,
                        dst_ip,
        })
    }

    pub fn parse_packet(buf: &[u8]) -> Result<Packet, ParseError>
    {
        let mut reader: BinaryReader = BinaryReader::new(buf);
        let eth: EthernetHeader = parse_ethernet(&mut reader)?;

        match eth.ethertype
        {
            EtherType::Ipv4 => {}
            _ => { panic!("Only IPv4 supported in this example"); }
        }

        let ip: Ipv4Header = parse_ipv4(&mut reader)?;
        let payload_len: usize = ip.total_length as usize - (ip.ihl as usize * 4);
        let payload: Vec<u8> = reader.read_bytes(payload_len)?.to_vec();

        Ok( Packet { ethernet: eth, ipv4: ip, payload })
    }

    pub fn demo()
    {
        let raw_packet: Vec<u8> = vec![
            // Ethernet
            0xff,0xff,0xff,0xff,0xff,0xff, // dst mac
            0x00,0x11,0x22,0x33,0x44,0x55, // src mac
            0x08,0x00,                     // IPv4

            // IPv4
            0x45,                         // version=4, ihl=5
            0x00,
            0x00,0x1c,                    // total length = 28
            0x00,0x01,
            0x00,0x00,
            64,                           // TTL
            17,                           // UDP
            0x00,0x00,
            192,168,0,1,                  // src ip
            192,168,0,2,                  // dst ip

            // Payload (8 bytes)
            1,2,3,4,5,6,7,8,
        ];

        match parse_packet(&raw_packet)
        {
            Ok(pkt) => { println!("{}", pkt); }
            Err(e) => { println!("parse error: {:?}", e); }
        }
    }
}


pub fn test_all()
{
    network_packet_parser::demo();

    // ================ PACKET ================
    // Ethernet:
    //   Src MAC : 00:11:22:33:44:55
    //   Dst MAC : ff:ff:ff:ff:ff:ff
    //   Type    : IPv4 (0x0800)
    //
    // IPv4
    //   Src IP  : 192.168.0.1
    //   Dst IP  : 192.168.0.2
    //   Proto   : UDP
    //   Length  : 28
    //
    // Payload (8 bytes)
    //   [01, 02, 03, 04, 05, 06, 07, 08]
    // ========================================
}