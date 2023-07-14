use crate::{VALID_URL_LETTERS};
use smaz::compress;
use num_bigint::BigUint;


pub fn smaz_compress(url: &str) -> String {
    let compressed = compress(&url.as_bytes());
    let mut num = BigUint::new(vec![0]);
    for i in 0..compressed.len() {
        num += (compressed[i] as u64) << (i * 8);
    }


    let base = VALID_URL_LETTERS.len();
    let mut digits = Vec::new();
    while num > BigUint::new(vec![0]) {
        digits.push(num % base);
        num /= base;
    }

    return digits.iter().map(|x| VALID_URL_LETTERS.chars().nth(*x as usize).unwrap()).collect::<String>();
}

pub fn smaz_decompress(url: &str) -> String {
    return std::str::from_utf8(&smaz::decompress(&valid_url_to_bin(url)).unwrap()).unwrap().to_string();
}