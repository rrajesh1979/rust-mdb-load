use crate::{Opt, INSERTS_N, QUERIES_N, UPDATES_N};
use std::error::Error;
use std::time::Duration;
use tokio::time;

const REPORT_FREQUENCY: u64 = 5000; //Report every 5 seconds

//TODO - there should be a better way to gather and print metrics
#[tokio::main]
pub async fn print_stats(opt: Opt) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut elapsed_seconds: i64 = 0;
    let duration: i64 = opt.duration as i64;
    let start_time = chrono::Utc::now();

    while elapsed_seconds <= duration {
        //TODO make interval configurable
        time::sleep(Duration::from_millis(REPORT_FREQUENCY)).await;
        info!(
            "------------ Stats after {} seconds -----------",
            elapsed_seconds
        );
        info!("Number of inserts: {}", INSERTS_N.lock().unwrap());
        info!("Number of updates: {}", UPDATES_N.lock().unwrap());
        info!("Number of queries: {}", QUERIES_N.lock().unwrap());
        info!("-----------------------------------------------");
        elapsed_seconds = chrono::Utc::now().timestamp() - start_time.timestamp();
    }

    Ok(())
}
