use std::panic;
use sycamore::prelude::*;
use web_sys;

mod compress;
use compress::Url;
mod smaz_cmp;

pub const VALID_URL_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_~[]@!$'()*,;";


fn get_params() -> Option<String> {
    let window = match web_sys::window() {Some(a)=>a,None=>return None};
    let url = match window.location().as_string() {Some(a)=>a,None=>return None};
    let params = match web_sys::UrlSearchParams::new_with_str(&url) {Ok(a)=>a,_=>return None};
    let value = match params.get("i") {Some(a)=>a,None=>return None};
    return Some(value)
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    if let Some(value) = get_params() {
        let new_url = Url::from_compressed_url(&value);
        if let Some(new_url) = new_url {
            web_sys::window().unwrap().location().set_href(&new_url.url).unwrap();
        }
    }

    sycamore::render(|cx|{
    let value = create_signal(cx, String::new());
    let new_url = create_signal(cx, String::new());
        
    view! { cx,
        input(bind:value=value)
        p {(new_url.get())}
    }



});
}