use std::env;
use openssl::ssl::{SslConnector, SslMethod};

pub fn test_all()
{
    if let Ok(v) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&v, 16).unwrap();
        if version >= 0x1_01_01_00_0 {
            println!("cargo:rustc-cfg=openssl111");
        }
    }
    else {
        println!("OpenSSL version number is invalid");
    }
}