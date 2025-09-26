use std::env;
use openssl::ssl::{SslConnector, SslMethod};

mod generate
{
    use openssl::sign::{Signer, Verifier};
    use openssl::rsa::Rsa;
    use openssl::pkey::{PKey, Private};

    pub fn generate_RSA_KeyPair()
    {
        let keypair: Rsa<Private> = Rsa::generate(2048).unwrap();
        let pkey: PKey<Private> = PKey::from_rsa(keypair.clone()).unwrap();

        println!("keypair: {:?}", &keypair);
        println!("pkey   : {:?}", &pkey);
    }
}

mod signing
{
    use openssl::sign::{Signer, Verifier};
    use openssl::rsa::Rsa;
    use openssl::pkey::{PKey, Private};
    use openssl::hash::MessageDigest;

    // https://docs.rs/openssl/latest/openssl/sign/index.html
    pub fn sign_and_verify ()
    {
        // Generate a keypair
        let keypair: Rsa<Private> = Rsa::generate(2048).unwrap();
        let keypair: PKey<Private> = PKey::from_rsa(keypair).unwrap();

        let data = b"hello, world!";
        let data2 = b"hola, mundo!";

        // Sign the data
        let mut signer: Signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
        signer.update(data).unwrap();
        signer.update(data2).unwrap();
        let signature: Vec<u8> = signer.sign_to_vec().unwrap();

        // Verify the data
        let mut verifier: Verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
        verifier.update(data).unwrap();
        verifier.update(data2).unwrap();
        assert!(verifier.verify(&signature).unwrap());
    }
}

mod encryption__rsa
{
    use openssl::encrypt::{Encrypter,Decrypter};
    use openssl::rsa::{Rsa, Padding};
    use openssl::pkey::{PKey, Private};

    pub fn encrypt_and_decrypt()
    {
        let keypair: Rsa<Private> = Rsa::generate(2048).unwrap();
        let pkey_private: PKey<Private> = PKey::from_rsa(keypair.clone()).unwrap();

        let mut encrypter: Encrypter = Encrypter::new(&pkey_private).unwrap();
        encrypter.set_rsa_padding(Padding::PKCS1).unwrap();

        let mut decrypter: Decrypter = Decrypter::new(&pkey_private).unwrap();
        decrypter.set_rsa_padding(Padding::PKCS1).unwrap();

        let data = b"hello, world!";

        let buffer_length: usize = encrypter.encrypt_len(data).unwrap();
        let mut encrypted_data: Vec<u8> = Vec::with_capacity(buffer_length);

        let encrypted_len: usize = encrypter.encrypt(data, &mut encrypted_data).unwrap();
        encrypted_data.truncate(encrypted_len);

        println!("encrypted_len: {:}", &encrypted_len);

        let buffer_len: usize = decrypter.decrypt_len(&encrypted_data).unwrap();
        let mut decrypted: Vec<u8> = Vec::with_capacity(buffer_len);

        let decrypted_len: usize = decrypter.decrypt(encrypted_data.as_slice(), &mut decrypted).unwrap();
        decrypted.truncate(decrypted_len);
    }
}

pub fn test_all()
{
    // generate::generate_RSA_KeyPair();
    // signing::sign_and_verify();
    // encryption__rsa::sign_and_verify();
}