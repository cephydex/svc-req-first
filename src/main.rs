mod config;
mod xtend;
mod xcheck;
mod api;

#[tokio::main]
async fn main() {

    // xcheck::bundle_call();
    xcheck::lookup_url_as().await;
    println!("ADHOC :: DONE\n\n");

    // let mut interval_timer = tokio::time::interval(chrono::Duration::days(1).to_std().unwrap());
    let mut interval_timer = tokio::time::interval(chrono::Duration::seconds(20).to_std().unwrap());
    loop {
        // Wait for the next interval tick
        interval_timer.tick().await;
        tokio::spawn(async { xtend::run_bundle().await; }); // For async task
        // tokio::task::spawn_blocking(|| xtend::run_bundle() ); // For blocking task
        // tokio::task::spawn_blocking(|| xtend::lookup_url() ); // For blocking task
    }

}
