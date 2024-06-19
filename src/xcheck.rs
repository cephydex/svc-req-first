use futures::{stream, StreamExt};
use reqwest::{blocking::Client, StatusCode};
use tokio::task::{/*JoinError,*/ JoinSet};

use crate::{api::{self, config::exec_url_as}, config::{self, print_date, RespItem, URLS}};

fn lookup_url() {        
    let client = api::config::create_client();

    let mut resp_arr: Vec<config::RespItem> = Vec::new();
    for (i, url) in config::URLS.iter().enumerate() {
        let resp = client.get(format!("{}", url)).send();
        if resp.is_ok() {
            let r = resp.as_ref();
            if r.unwrap().status() != StatusCode::OK {
                resp_arr.push(config::RespItem{code: r.unwrap().status().as_str().parse::<i32>().unwrap(), site: url.to_string()});
           } else {
                println!("{}) All is well", (i +1));
           }
           
        } else if resp.is_err() {
            println!("responses :: {:#?}", resp.err().unwrap());
        }
    }
    println!("response list :: {:?}.", resp_arr);
}

pub fn bundle_call() {
    print_date();
    lookup_url();

    call_url();
}

pub async fn lookup_url_as() {
    let client = api::config::create_clienta().await;

    let mut resp_arr: Vec<config::RespItem> = Vec::new();
    for (i, url) in config::URLS.iter().enumerate() {
        let res = exec_url_as(url).await;
        resp_arr.push(res);
        // let resp = client
        //     .get(format!("{}", url))
        //     .send()
        //     .await;

        // if resp.is_ok() {
        //     let r = resp.as_ref();
        //     if r.unwrap().status() != StatusCode::OK {
        //         resp_arr.push(config::RespItem{code: r.unwrap()
        //             .status()
        //             .as_str()
        //             .parse::<i32>()
        //             .unwrap(), site: url.to_string()});
        //    } else {
        //         println!("{}) All is well", (i +1));
        //    }
           
        // } else if resp.is_err() {
        //     println!("responses :: {:#?}", resp.err().unwrap());
        // }
    }
    println!("response list :: {:?}.", resp_arr);
}

pub fn call_url() {
    let client = Client::new();    
    let req = client.get("https://public-api.megafortunelottery.com/swagger/index.html").send();

    let _resp = match req {
        Ok(resp) => resp,
        Err(err) => {
            if err.is_timeout() {
                println!("encountered timeout");
                return;
            } else {
                panic!("{err}");
            }
        }
    };

    println!("response other :: {:?}.", _resp);
}

async fn do_the_hard_job(port: u16) -> u16 {
    println!("Hard job :: {}", port);
    port
}

async fn run_concur() {
    let inputs = stream::iter(0..=10);
    let port: u16 = 1200;
    use futures::lock::Mutex; // std::sync::Mutex not working in async
    let return_values: Vec<u16> = Vec::new();
    let return_values = Mutex::new(return_values); // wrap by Mutex
    inputs
        .for_each_concurrent(0, |_input| async { // remove 'move' keyword
        
            if let done = do_the_hard_job(port).await {
                if done > 0 {
                    // println!("Input item {}", input);
                    let mut return_values = return_values.lock().await; // sync
                    return_values.push(port);
                }
            }
                // println!("Processed value: {}", input);
        })
        .await;
}


// pub async fn run_concur_two()  -> Result<Vec<String>, reqwest::Error> {
// pub async fn run_concur_two() -> Result<RespItem, reqwest::Error> {
// pub async fn run_concur_two() -> Result<JoinSet<RespItem>, JoinError> {
async fn run_concur_two() -> Vec<RespItem> {
    // let mut handles = vec![];
    let mut results: Vec<config::RespItem> = Vec::new();
    let mut set: JoinSet<RespItem> = JoinSet::new();
    
    for url in URLS.iter() {
        set.spawn(exec_url_as(url));
    }

    while let Some(res) = set.join_next().await {
        let out = res;

        let _ = match out {
            Ok(resp) => {
                // println!("success :: {:#?}", resp);
                if resp.code != 200 {
                    results.push(resp);
                }
            },
            Err(err) => {
                println!("error :: {:#?}", err);
            }
        };
    }

    return results;
}

// fn ConcurrentTask() {
//     let mut timer1 = Timer::new().unwrap();
//     let mut timer2 = Timer::new().unwrap();
//     let tick1 = timer1.periodic(Duration::from_secs(20));
//     let tick2 = timer2.periodic(Duration::seconds(3));

//     loop {
//         select! {
//             _ = tick1.recv() => do_something1(),
//             _ = tick2.recv() => do_something2()
//         }
//     }
// }



// let mut interval_timer = tokio::time::interval(chrono::Duration::seconds(20).to_std().unwrap());
// loop {
//     // Wait for the next interval tick
//     interval_timer.tick().await;
//     tokio::spawn(async { do_my_task().await; }); // For async task
//     tokio::task::spawn_blocking(|| do_my_task()); // For blocking task
// }