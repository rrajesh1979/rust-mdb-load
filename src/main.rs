#[macro_use]
extern crate log;
extern crate log4rs;

use clap::lazy_static::lazy_static;
use clap::Parser;
use cli_helper::Opt;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use mongo_util::FieldTypes::{Date, Int, Text};
use mongo_util::Ops::{Insert, Query, Update};
use std::sync::Mutex;
use std::thread;

mod cli_helper;
mod mongo_load_gen;
mod mongo_util;
mod stats_reporter;

//TODO Validate if the concurrent access is implemented idiomatically. Do I need to use Arc<T> ?
//TODO use a better data structure instead of individual ?
lazy_static! {
    pub static ref INSERTS_N: Mutex<i32> = Mutex::new(0i32);
    pub static ref QUERIES_N: Mutex<i32> = Mutex::new(0i32);
    pub static ref UPDATES_N: Mutex<i32> = Mutex::new(0i32);
    pub static ref QUERIES: &'static str = "Queries";
    pub static ref INSERTS: &'static str = "Inserts";
    pub static ref UPDATES: &'static str = "Updates";
    pub static ref INSERTS_SLOW: Mutex<i32> = Mutex::new(0i32);
    pub static ref QUERIES_SLOW: Mutex<i32> = Mutex::new(0i32);
    pub static ref UPDATES_SLOW: Mutex<i32> = Mutex::new(0i32);
    pub static ref ELAPSED_TIME: Mutex<i64> = Mutex::new(0i64);
}

fn main() {
    initialize_logging();
    info!("Initializing MongoDB load generator!");

    // Parse CLI options
    let opt: Opt = Opt::parse();
    let threads = opt.threads;

    // Initialize DB
    let init_opt = opt.clone();
    let _init_result = mongo_load_gen::mongodb_init(init_opt);

    let stat_opt = opt.clone();
    let stats_handle = thread::spawn(move || {
        let _stats_result = stats_reporter::print_stats(stat_opt);
    });

    let mut handles = Vec::new();

    for i in 0..threads {
        let new_opt = opt.clone();
        let handle = thread::spawn(move || {
            let _result = mongo_load_gen::mongodb_load_gen(new_opt, i, opt.run_id_start);
        });
        handles.push(handle);
    }
    handles.push(stats_handle);

    for handle in handles {
        let _result = handle.join().unwrap();
    }

    stats_reporter::print_slow_ops();
}

fn initialize_logging() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
}
