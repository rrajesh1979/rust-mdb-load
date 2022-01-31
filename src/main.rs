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

lazy_static! {
    pub static ref INSERTS_N: Mutex<i32> = Mutex::new(0i32);
    pub static ref QUERIES_N: Mutex<i32> = Mutex::new(0i32);
    pub static ref UPDATES_N: Mutex<i32> = Mutex::new(0i32);
}

fn main() {
    initialize_logging();
    info!("Initializing MongoDB load generator!");

    // Parse CLI options
    let opt: Opt = Opt::parse();

    let stats_handle = thread::spawn(|| {
        let stat_opt = Opt::parse();
        let _stats_result = mongo_load_gen::print_stats(stat_opt);
    });

    let mut handles = Vec::new();

    for i in 1..opt.threads + 1 {
        let handle = thread::spawn(move || {
            let new_opt = Opt::parse();
            let _result = mongo_load_gen::mongodb_load_gen(new_opt, i, opt.run_id_start);
        });
        handles.push(handle);
    }
    handles.push(stats_handle);

    for handle in handles {
        let _result = handle.join().unwrap();
    }
}

fn initialize_logging() {
    let stdout = ConsoleAppender::builder().build();
    // log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
}
