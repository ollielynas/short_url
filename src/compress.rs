

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::VALID_URL_LETTERS;
use crate::smaz_cmp::smaz_compress;
use crate::smaz_cmp::smaz_decompress;
use crate::yt::{yt_compress, yt_decompress};


pub struct Url {
    pub url: String,
    pub compressed_url: String,
}




#[derive(EnumIter)]
enum Compression {
    None,
    Smaz,
    YouTube
}

impl Compression {
    fn compress(&self, url: &str) -> String {
        match self {
            Compression::None => url.to_string(),
            Compression::Smaz => smaz_compress(url),
            Compression::YouTube => yt_compress(url),

        }
    }

    fn decompress(&self, url: &str) -> Option<String> {
        match self {
            Compression::None => Some(url.to_string()),
            Compression::Smaz => smaz_decompress(url),
            Compression::YouTube => yt_decompress(url),
        }
    }
}



impl Url {
    pub fn from_url(url: &str) -> Url {
        let chars = VALID_URL_LETTERS.chars().collect::<Vec<char>>();
        let mut methods = Compression::iter().enumerate().map(|(i, x)| (chars[i], x.compress(url))).collect::<Vec<(char,String)>>();
        methods.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

        return Url {
            url: url.to_string(),
            compressed_url: format!("?i={}{}",methods[0].0, methods[0].1),
        }
    }

    pub fn from_compressed_url(compressed_url: &str) -> Option<Url> {
        let chars = VALID_URL_LETTERS.chars().collect::<Vec<char>>();
        // remove first letter of string
        if compressed_url.len() < 1 {
            return None;
        }
        let mut url = compressed_url.to_string();
        let compression_type = url.remove(0);
        let mut methods = Compression::iter();
        let compression_index = chars.iter().position(|&r| r == compression_type).unwrap_or(0);
        
        if let Some(compression) = methods.nth(compression_index) {
            if let Some(decompressed) = compression.decompress(&url) {
                return Some(Url {
                    url:decompressed,
                    compressed_url: compressed_url.to_owned(),
                });
            }else {
                return None;
            }
            
        }else {
            return None;
        }
    }
}