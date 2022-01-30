use std::error::Error;
use mongodb::Client;
use crate::{mongo_util, Opt};

#[tokio::main]
pub async fn mongodb_load_gen(opt: Opt) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = Client::with_uri_str(opt.conn).await?;
    let namespace = opt.namespace;

    let (db, coll) = parse_namespace(&namespace);

    let database = client.database(&*db);
    let collection = database.collection(&*coll);
    let mut elapsed_seconds: i64 = 0;
    let start_time = chrono::Utc::now();

    //TODO Worked around the Reference issue by splitting into scalar variables. Need to find out the right way to do
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
        collection.insert_one(mongo_util::create_doc(num_fields, depth, txt_len, binary), None).await?;
        elapsed_seconds = chrono::Utc::now().timestamp() - start_time.timestamp();
    }
    Ok(())
}

fn parse_namespace(ns: &String) -> (String, String) {
    let namespace: Vec<&str> = ns.split(".").collect();
    let db;
    let coll;
    if namespace.len() == 2 {
        db = namespace[0];
        coll = namespace[1];
    } else {
        db = "rmdb";
        coll = "load";
    }
    (db.parse().unwrap(), coll.parse().unwrap())
}

