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

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error + Send + Sync>> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("Initializing MongoDB load generator!");

    let opt: Opt = Opt::parse();
    println!("{:#?}", opt);

    mongo_load_gen::mongodb_load_gen(opt).await

    // let handle = thread::spawn(|| {
    //
    // });
    // handle.join().unwrap();
}
