use crate::{Opt, INSERTS_N, INSERTS_SLOW, QUERIES_N, QUERIES_SLOW, UPDATES_N, UPDATES_SLOW};
use chrono::Duration as chrono_duration;
use std::error::Error;
use std::time::Duration;
use tokio::time;

const REPORT_FREQUENCY: u64 = 5000; //Report every 5 seconds

pub fn record_ops_done(op_type: &str, count: i32) {
    match op_type {
        "Inserts" => {
            //Increment number of insert ops
            let mut insert_num = INSERTS_N.lock().unwrap();
            *insert_num += count;
        }
        "Updates" => {
            //Increment number of update ops
            let mut update_num = UPDATES_N.lock().unwrap();
            *update_num += count;
        }
        "Queries" => {
            //Increment number of insert ops
            let mut query_num = QUERIES_N.lock().unwrap();
            *query_num += count;
        }
        _ => {}
    };
}

pub fn record_slow_ops(op_type: &str, _threshold: &chrono_duration) {
    match op_type {
        "Inserts" => {
            //Increment number of insert ops
            let mut insert_slow = INSERTS_SLOW.lock().unwrap();
            *insert_slow += 1;
        }
        "Updates" => {
            //Increment number of update ops
            let mut update_slow = UPDATES_SLOW.lock().unwrap();
            *update_slow += 1;
        }
        "Queries" => {
            //Increment number of insert ops
            let mut query_slow = QUERIES_SLOW.lock().unwrap();
            *query_slow += 1;
        }
        _ => {}
    };
}

pub fn print_slow_ops() {
    info!("------------ Slow Ops during the run-----------");
    info!("Number of slow inserts: {}", INSERTS_SLOW.lock().unwrap());
    info!("Number of slow updates: {}", UPDATES_SLOW.lock().unwrap());
    info!("Number of slow queries: {}", QUERIES_SLOW.lock().unwrap());
    info!("-----------------------------------------------");
}

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
