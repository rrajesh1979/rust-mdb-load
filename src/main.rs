#[macro_use]
extern crate log;
extern crate log4rs;
extern crate byte_unit;

use std::error::Error;
use byte_unit::n_mb_bytes;
use chrono::prelude::*;
use clap::Parser;
use lipsum::lipsum;
use mongodb::bson::{doc, Document, Uuid};
use mongodb::{Client};
use crate::FieldTypes::{TypeDate, TypeInt, TypeText};
use rand::{Rng};
use serde::{Deserialize, Serialize};

/// Load generator for MongoDB Atlas built using Rust
#[derive(Parser, Debug)]
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

    mongodb_load_gen(opt).await
}

pub async fn mongodb_load_gen(opt: Opt) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = Client::with_uri_str("mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false").await?;
    let database = client.database("rmdb");
    let collection = database.collection("load");
    collection.insert_one(create_doc(opt), None).await?;
    Ok(())
}

fn create_doc(opt: Opt) -> Document {
    let mut mongo_doc = MongoDoc {
        document: Document::new(),
        num_fields: opt.num_fields,
        depth: 0,
        txt_len: opt.text_size,
        binary: opt.binary,
    };
    mongo_doc.add_id();
    mongo_doc.add_fields();
    mongo_doc.add_binary();
    mongo_doc.document
}

fn create_string(len: usize) -> String {
    let random_text = lipsum(len);
    random_text.to_string()
}

#[derive(Serialize, Deserialize)]
struct MongoDoc {
    document: Document,
    num_fields: u16,
    depth: usize,
    txt_len: usize,
    binary: Option<bool>,
}

trait Create {
    fn add_fields(&mut self);
    fn add_id(&mut self);
    fn add_binary(&mut self);
    fn field_type(&self, field_num: u16) -> FieldTypes;
}

impl Create for MongoDoc {
    fn add_fields(&mut self) {
        let mut field_num: u16 = 0;
        while field_num < self.num_fields {
            let f_type = self.field_type(field_num);
            let i: u32 = rand::thread_rng().gen();
            let t: DateTime<Utc> = Utc::now();

            let s = create_string(self.txt_len);
            match f_type {
                TypeInt => self.document.insert(format!("{}{}", "fld", field_num), i),
                TypeDate => self.document.insert(format!("{}{}", "fld", field_num), chrono::Utc::now() ),
                _ => self.document.insert(format!("{}{}", "fld", field_num), s),
            };
            field_num += 1;
        }
    }

    fn add_id(&mut self) {
        self.document.insert("_id", Uuid::new());
    }

    fn add_binary(&mut self) {
        match Some(self.binary) {
            None => {
                info!("Inside None");
            }
            Some(_) => {
                info!("Inside Some");
                let result = n_mb_bytes(2);
                self.document.insert(format!("{}{}", "fld", "_binary"), "result");
            }
        };
    }

    fn field_type(&self, field_num: u16) -> FieldTypes {
        if field_num == 0 {
            TypeInt
        } else if field_num == 1 {
            TypeDate
        } else if field_num == 3 {
            TypeText
        } else if field_num % 3 == 0 {
            TypeInt
        } else if field_num % 5 == 0 {
            TypeDate
        } else {
            TypeText
        }
    }
}

enum FieldTypes {
    TypeInt,
    TypeDate,
    TypeText,
}
