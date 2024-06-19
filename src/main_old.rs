use core::fmt;
use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::StatusCode;

mod config;
mod xtras;

fn main() {

    struct RespItem {
        site: String,
        code: i32
    }

    impl fmt::Debug for RespItem {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.debug_struct("RespItem")
                .field("site", &self.site)
                // .field("code", &self.code)
                .field("code", &format_args!("{}", self.code))
                .finish()
        }
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(4))
        .build()
        .unwrap();

    let mut resp_arr: Vec<RespItem> = Vec::new();
    // let mut check_str: Vec<String> = Vec::new();
    for (i, url) in config::urls.iter().enumerate() {
    // for url in urls {
        // let url_c: String = format!("{}", url);
        let resp = client.get(format!("{}", url)).send();
        if resp.is_ok() {
            // check_str.push(url.to_string());
            let r = resp.as_ref();
            if r.unwrap().status() != StatusCode::OK {
                // println!("{:#?}", r.ok().unwrap().url().host());
                resp_arr.push(RespItem{code: r.unwrap().status().as_str().parse::<i32>().unwrap(), site: url.to_string()});
           } else {
                println!("{}) All is well", (i +1));
           }
           
        } else if resp.is_err() {
            println!("responses :: {:#?}", resp.err().unwrap());
        }
    }
    println!("response list :: {:?}.", resp_arr);

    xtras::call_url()

}
