use std::panic;
use sycamore::prelude::*;
use web_sys;
use num_bigint::BigUint;

mod compress;
use compress::Url;
mod smaz_cmp;
mod yt;


pub const VALID_URL_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_~[]@!$()*,;";


fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    if let Ok(value) = web_sys::window().unwrap().location().search() {
        
        let new_url = Url::from_compressed_url(&value.replace("?i=", ""));
        if let Some(new_url) = new_url {
            // web_sys::window().unwrap().location().set_href(&new_url.url).unwrap();
            web_sys::window().unwrap().location().replace(&format!("https://{}",&new_url.url)).unwrap();
        }
    }    
    
    
    sycamore::render(|cx|{
    let current_url = web_sys::window().unwrap().location().href().unwrap();
    let value = create_signal(cx, String::new());
    let new_url = create_signal(cx, String::new());
    
    view! { cx,
        div {
        h1(){("URL Shortener")}
        textarea(bind:value=value)
        p {(new_url.get())}
        button(on:click = move |_| {
            new_url.set(format!("{}{}", current_url.replace("https://", "").replace("http://", ""), Url::from_url(&value.get().replace("https://", "").replace("http://", "")).compressed_url));
        }) {("convert")}
    }
    }



});
}