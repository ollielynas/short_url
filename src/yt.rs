

pub fn yt_compress(url: &str) -> String {
    let mut out=url.to_owned();
    if url.starts_with("www.youtube.com/shorts/") {
        out = url.replace("www.youtube.com/shorts/", "s");
    }
    else if url.starts_with("www.youtube.com/watch?v=") {
        out = url.replace("www.youtube.com/watch?v=", "v");
    }
    else if url.starts_with("stackoverflow.com/questions/") {
        out = url.replace("stackoverflow.com/questions/", "t");
        out = out.split("/").collect::<Vec<&str>>()[0].to_owned();
    }
    else {
        out = "n".to_owned() + &out;
    }
    return out;
}

pub fn yt_decompress(url: &str) -> Option<String> {
    if url.starts_with("s") {
        Some(url.replace("s", "www.youtube.com/shorts/"))
    }
    else if url.starts_with("v") {
        Some(url.replace("v", "youtube.com/watch?v="))
    }
    else if url.starts_with("n") {
        Some(url.replace("n", ""))
    }
    else if url.starts_with("t") {
        Some(url.replace("t", "stackoverflow.com/questions/"))
    }
    else {
        None
    }
}