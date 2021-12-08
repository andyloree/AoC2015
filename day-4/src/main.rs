use std::time::{Instant};
use md5;

fn find_md5_starts_with(secret: String, starts_with: &str) -> (i32, String) {
    let mut hash = String::new();
    let mut i: i32 = 0;
    while !hash.starts_with(&starts_with) {
        i += 1;
        let str = format!("{}{}", secret, i);
        let digest = md5::compute(str);
        hash = format!("{:x}", digest).to_owned();
    }
    return (i, hash)
}

fn main() {
    let start = Instant::now();
    let secret = "yzbqklnj";
    
    let (i, hash) = find_md5_starts_with(secret.to_string(), "00000");

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Secret: {}", secret);
    println!("Answer: {}", i);
    println!("Hash: {}\r\n", hash);

    let (i, hash) = find_md5_starts_with(secret.to_string(), "000000");
    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Secret: {}", secret);
    println!("Answer: {}", i);
    println!("Hash: {}\r\n", hash);

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
}