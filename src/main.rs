#[macro_use]
extern crate log;
extern crate log4rs;

use std::error::Error;
use clap::Parser;
use mongodb::bson::{doc, Document};
use mongodb::{Client, options::ClientOptions};

/// Load generator for MongoDB Atlas built using Rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Opt {
    /// MongoDB connection string
    #[clap(short, long)]
    conn: Option<String>,

    /// Duration in seconds. Default 120
    #[clap(short, long, parse(try_from_str), default_value_t = 120)]
    duration: usize,

    /// Number of top level fields in the document. Default 10
    #[clap(short, long, parse(try_from_str), default_value_t = 10)]
    num_fields: usize,

    /// Nesting depth. Default 0
    #[clap(long, parse(try_from_str), default_value_t = 0)]
    nest_depth: usize,

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

    let opt = Opt::parse();
    println!("{:#?}", opt);

    mongodb_load_gen().await
}

pub async fn mongodb_load_gen() -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = Client::with_uri_str("mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false").await?;
    let database = client.database("rmdb");
    let collection = database.collection("load");
    let document = doc! {
        "name": "rrajesh1979",
        "dob": 1979
    };
    collection.insert_one(document, None).await?;
    Ok(())
}