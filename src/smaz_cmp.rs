use crate::{VALID_URL_LETTERS};
use smaz::compress;
use num_bigint::BigUint;

pub fn smaz_compress(url: &str) -> String {
    let compressed = compress(&url.as_bytes());
    let num: BigUint = BigUint::from_bytes_be(&compressed);
    let base = VALID_URL_LETTERS.len();
    
    let arr = num.to_radix_be(base as u32);

    let result = arr.iter().map(|x| VALID_URL_LETTERS.chars().nth(*x as usize).unwrap()).collect::<String>();

    return result;
    

}

pub fn smaz_decompress(url: &str) -> Option<String> {
    let chars = url.chars().collect::<Vec<char>>();
    let base = VALID_URL_LETTERS.len();
    let num = BigUint::from_radix_be(&chars.iter().map(|x| VALID_URL_LETTERS.chars().position(|r| r == *x).expect(&format!("invalid character {}", x)) as u8).collect::<Vec<u8>>().as_slice(), base as u32).expect("invalid base");
    let mut bytes = num.to_bytes_be();
    let string = smaz::decompress(&bytes);
    return match string {
        Ok(string) => Some(String::from_utf8(string).unwrap()),
        Err(_) => None,
    }
}