use std::env;

pub fn http_provider_url() -> String {

    env::var("HTTP_PROVIDER_URL").expect("HTTP_PROVIDER_URL NOT FOUND")

}

pub fn ws_provider_url() -> String {

    env::var("WS_PROVIDER_URL").expect("WS_PROVIDER_URL NOT FOUND")

}

pub fn my_address() -> String {

    env::var("MY_ADDRESS").expect("MY_ADDRESS NOT FOUND")

}

pub fn my_private_key() -> String {
    
    env::var("PRIVATE_KEY").expect("PRIVATE_KEY NOT FOUND")

}