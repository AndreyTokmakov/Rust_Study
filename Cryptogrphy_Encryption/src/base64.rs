
mod openssl_base64
{
    use openssl::base64 as b64;
    use openssl::error::ErrorStack;

    pub fn encode()
    {
        let data_to_encode = b"Hello, Rust Base64!";
        let encoded_string: String = b64::encode_block(data_to_encode);
        println!("Original data: {:?}", data_to_encode);
        println!("Encoded string: {}", encoded_string);

        // Original data: [72, 101, 108, 108, 111, 44, 32, 82, 117, 115, 116, 32, 66, 97, 115, 101, 54, 52, 33]
        // Encoded string: SGVsbG8sIFJ1c3QgQmFzZTY0IQ==
    }

    pub fn decode()
    {
        let encoded_string = "SGVsbG8sIFJ1c3QgQmFzZTY0IQ=="; // "Hello, Rust Base64!" Base64 encoded
        let decoded_bytes = b64::decode_block(encoded_string).expect("Failed to decode Base64");
        println!("Encoded string: {}", encoded_string);
        println!("Decoded bytes: {:?}", decoded_bytes);
        println!("Decoded string: {}", String::from_utf8_lossy(&decoded_bytes));
    }
}

mod base64_tests
{
    use base64;

    pub fn encode_decode_old_style()
    {
        let text: String = String::from("Hello, Rust!");

        // Encode to base64
        let encoded = base64::encode(text);
        println!("Encoded: {}", encoded);

        // Decode back
        let decoded_bytes = base64::decode(&encoded).unwrap();
        let decoded = String::from_utf8(decoded_bytes).unwrap();
        println!("Decoded: {}", decoded);
    }
}

pub fn test_all()
{
    // openssl_base64::encode();
    // openssl_base64::decode();

    base64_tests::encode_decode_old_style();
}