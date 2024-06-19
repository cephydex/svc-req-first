use tokio::task::JoinSet;

use crate::{api::config::exec_url_as, config::{self, print_date, RespItem}};

pub async fn lookup_concur() -> Vec<RespItem> {
    // let mut handles = vec![];
    let mut results: Vec<config::RespItem> = Vec::new();
    let mut set: JoinSet<RespItem> = JoinSet::new();
    
    for url in config::URLS.iter() {
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

pub async fn run_bundle() {
    print_date();
    let tt = lookup_concur().await;
    // for item in tt.iter() {
    //     println!("Print CHECK >> {:?}", item);        
    // }

    println!("RES >> {:?}", tt);    
}
