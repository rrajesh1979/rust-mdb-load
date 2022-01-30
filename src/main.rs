#[macro_use]
extern crate log;
extern crate log4rs;

use std::error::Error;
use clap::Parser;
use mongodb::bson::{doc};
use mongodb::Client;
use mongo_worker::FieldTypes::{TypeDate, TypeInt, TypeText};
use std::thread;

mod mongo_worker;

/// Load generator for MongoDB Atlas built using Rust
#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Opt {
    /// MongoDB connection string
    #[clap(short, long)]
    conn: Option<String>,

    /// Duration in seconds. Default 120
    #[clap(short, long, parse(try_from_str), default_value_t = 120)]
    duration: usize,

    /// Number of top level fields in the document. Default 10
    #[clap(short, long, parse(try_from_str), default_value_t = 10)]
    num_fields: u16,

    /// Nesting depth. Default 0
    #[clap(long, parse(try_from_str), default_value_t = 0)]
    nest_depth: u8,

    /// Ratio of Inserts. Default 100
    #[clap(short, long, parse(try_from_str), default_value_t = 100)]
    inserts: usize,

    /// Ratio of Updates. Default 0
    #[clap(short, long, parse(try_from_str), default_value_t = 0)]
    updates: usize,

    /// Ratio of Queries. Default 0
    #[clap(short, long, parse(try_from_str), default_value_t = 0)]
    queries: usize,

    /// Length of text fields in bytes. Default 30
    #[clap(long, parse(try_from_str), default_value_t = 30)]
    text_size: usize,

    /// Namespace to use. Default rmdb.load
    #[clap(long, default_value = "rmdb.load")]
    namespace: String,

    /// Print sample document
    #[clap(short, long)]
    print: Option<bool>,

    /// Number of threads. Default 3
    #[clap(short, long, parse(try_from_str), default_value_t = 3)]
    threads: usize,

    /// Run ID start. Default is 0
    #[clap(short, long, parse(try_from_str), default_value_t = 0)]
    run_id_start: usize,

    /// Add BLOB filed
    #[clap(short, long)]
    binary: Option<bool>,
}

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error + Send + Sync>> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("Initializing MongoDB load generator!");

    let opt: Opt = Opt::parse();
    println!("{:#?}", opt);

    mongodb_load_gen(opt).await

    // let handle = thread::spawn(|| {
    //
    // });
    // handle.join().unwrap();
}

pub async fn mongodb_load_gen(opt: Opt) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = Client::with_uri_str("mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false").await?;
    let database = client.database("rmdb");
    let collection = database.collection("load");
    let mut elapsed_seconds: i64 = 0;
    let start_time = chrono::Utc::now();
    let duration: i64 = opt.duration as i64;
    let mut binary = false;
    match opt.binary {
        None => {
        }
        Some(_) => {
            binary = true
        }
    };
    let txt_len = opt.text_size;
    let depth = opt.nest_depth;
    let num_fields = opt.num_fields;
    while elapsed_seconds < duration {
        collection.insert_one(mongo_worker::create_doc(num_fields, depth, txt_len, binary), None).await?;
        elapsed_seconds = chrono::Utc::now().timestamp() - start_time.timestamp();
    }
    Ok(())
}
