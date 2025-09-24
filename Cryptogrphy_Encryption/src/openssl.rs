use std::env;
use openssl::ssl::{SslConnector, SslMethod};

mod signing
{
    use openssl::sign::{Signer, Verifier};
    use openssl::rsa::Rsa;
    use openssl::pkey::PKey;
    use openssl::hash::MessageDigest;

    // https://docs.rs/openssl/latest/openssl/sign/index.html
    pub fn sign_and_verify ()
    {
        // Generate a keypair
        let keypair = Rsa::generate(2048).unwrap();
        let keypair = PKey::from_rsa(keypair).unwrap();

        let data = b"hello, world!";
        let data2 = b"hola, mundo!";

        // Sign the data
        let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
        signer.update(data).unwrap();
        signer.update(data2).unwrap();
        let signature = signer.sign_to_vec().unwrap();

        // Verify the data
        let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
        verifier.update(data).unwrap();
        verifier.update(data2).unwrap();
        assert!(verifier.verify(&signature).unwrap());
    }
}

pub fn test_all()
{
    /*
    if let Ok(v) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&v, 16).unwrap();
        if version >= 0x1_01_01_00_0 {
            println!("cargo:rustc-cfg=openssl111");
        }
    }
    else {
        println!("OpenSSL version number is invalid");
    }
    */

    signing::sign_and_verify();
}