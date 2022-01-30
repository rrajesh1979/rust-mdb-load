#[macro_use]
extern crate log;
extern crate log4rs;

use clap::Parser;
use cli_helper::Opt;
use mongo_util::FieldTypes::{TypeDate, TypeInt, TypeText};
use mongo_util::Ops::{MDBInsert, MDBQuery, MDBUpdate};
use std::error::Error;
use std::thread;

mod cli_helper;
mod mongo_load_gen;
mod mongo_util;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Initializing MongoDB load generator!");
    let opt: Opt = Opt::parse();

    let mut handles = Vec::new();

    for i in 1..opt.threads {
        let handle = thread::spawn(move || {
            let new_opt = Opt::parse();
            let _result = mongo_load_gen::mongodb_load_gen(new_opt, i, opt.run_id_start);
        });
        handles.push(handle);
    }

    for handle in handles {
        let _result = handle.join().unwrap();
    }
}
