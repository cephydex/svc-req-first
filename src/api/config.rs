use std::error::Error;
use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::{Client as async_client, StatusCode};

use crate::config::RespItem;

pub fn create_client() -> Client {
    let client = Client::builder()
            .timeout(Duration::from_secs(4))
            .build()
            .unwrap();

    return client;
}

pub async fn create_clienta() -> async_client {
    return reqwest::Client::builder()
        .timeout(Duration::from_secs(3)).build().unwrap()

    // return reqwest::Client::new();
}

pub async fn exec_url_as(url: &str) -> RespItem {
    let client = super::config::create_clienta().await;
    let mut item = RespItem{site: String::new(), code: 200};

    let resp = client
        .get(format!("{}", url))
        .send()
        .await;

    if resp.is_ok() {
        let r = resp.as_ref();
        if r.unwrap().status() != StatusCode::OK {
            item = RespItem{code: r.unwrap()
                .status()
                .as_str()
                .parse::<i32>()
                .unwrap(), site: url.to_string()};
        } else {
            println!("REQ :: {}", url);
        }
        
    } else if resp.is_err() {
        let rr = resp.as_ref();
        println!("ERR RESP :: {:#?}", rr.err().unwrap());
        println!("ERR RESP sources :: {:#?}", rr.err().unwrap().source());
        // item = RespItem{code: rr.unwrap()
        //     .status()
        //     .as_str()
        //     .parse::<i32>()
        //     .unwrap(), site: url.to_string()};
    }
    
    return item;
}
