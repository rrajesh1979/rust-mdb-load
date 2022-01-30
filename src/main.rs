#[macro_use]
extern crate log;
extern crate log4rs;

use std::error::Error;
use clap::Parser;
use mongo_util::FieldTypes::{TypeDate, TypeInt, TypeText};
use std::thread;
use cli_helper::Opt;

mod mongo_util;
mod cli_helper;
mod mongo_load_gen;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Initializing MongoDB load generator!");
    let opt: Opt = Opt::parse();

    mongo_load_gen::mongodb_load_gen(opt);

    // let handle = thread::spawn(|| {
    //
    // });
    // handle.join().unwrap();
}
