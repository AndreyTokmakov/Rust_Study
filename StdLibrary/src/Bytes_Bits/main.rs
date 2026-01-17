

fn print_as_binary()
{
    let a: u16 = 10115;
    let b: i16 = -15421;

    println! ("a: {:016b} {}", a, a);
    println! ("b: {:016b} {}", b, b);
    
    // а: 1100001111000011 50115
    // b: 1100001111000011 -15421
}


fn byte_array_to_integer()
{
    let bytes: [u8; 2] = [0x01, 0x02];
    let value: u16 = u16::from_be_bytes(bytes);
    println!("{}", value); // 258
}


mod network_data_tests
{
    fn get_port(buf: &[u8]) -> u16
    {
        let bytes: [u8; 2] = [buf[0], buf[1]];
        u16::from_be_bytes(bytes)
    }

    pub fn parse_port()
    {
        let packet: [u8; 2] = [0x01, 0xBB];
        let port: u16 = get_port(&packet);
        println!("port = {}", port);
    }
}

mod reading_byte_slice
{
    fn read_as_u32(buf: &[u8], offset: usize) -> u32
    {
        let bytes: [u8; 4] = buf[offset..offset + 4].try_into().unwrap();
        return u32::from_be_bytes(bytes)
    }

    pub fn read_u32_data()
    {
        let data: [u8; 4] = [0x00, 0x00, 0x01, 0x00];
        let value: u32 = read_as_u32(&data, 0);

        println!("{}", value); // 256
    }
}

pub fn test_all()
{
    // print_as_binary();
    // byte_array_to_integer();

    // network_data_tests::parse_port();
    reading_byte_slice::read_u32_data();
}