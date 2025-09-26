
use sha2::{ Sha256, Sha384, Sha512, Digest };

fn compute_hash_sha_256()
{
    let mut hasher = Sha256::new();
    hasher.update(b"hello, world!");
    let hash = hasher.finalize();

    println!("Sha-256 hash: {:?}", hash);
    println!("Sha-256 hash (Hex): {:x}", hash);

    // Sha-256 hash: [104, 230, 86, 178, 81, 230, 126, 131, 88, 190, 248, 72, 58, 176,
    //      213, 28, 102, 25, 243, 231, 161, 169, 240, 231, 88, 56, 212, 31, 243, 104, 247, 40]
    // Sha-256 hash (Hex): 68e656b251e67e8358bef8483ab0d51c6619f3e7a1a9f0e75838d41ff368f728
}

pub fn test_all()
{
    compute_hash_sha_256();
}