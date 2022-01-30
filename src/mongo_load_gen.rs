use crate::{mongo_util, Opt};
use bson::doc;
use chrono::Utc;
use mongodb::Client;
use std::collections::HashMap;
use std::error::Error;

use crate::{MDBInsert, MDBQuery, MDBUpdate};
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;

#[tokio::main]
pub async fn mongodb_load_gen(
    opt: Opt,
    process_id: usize,
    run_id_start: usize,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = Client::with_uri_str(opt.conn).await?;
    let namespace = opt.namespace;

    let (db, coll) = parse_namespace(&namespace);

    let database = client.database(&*db);
    let collection = database.collection(&*coll);
    let mut elapsed_seconds: i64 = 0;
    let start_time = chrono::Utc::now();

    let op_weight = [
        (MDBInsert, opt.inserts),
        (MDBQuery, opt.queries),
        (MDBUpdate, opt.updates),
    ];
    let dist = WeightedIndex::new(op_weight.iter().map(|item| item.1)).unwrap();
    let mut rng = thread_rng();

    let mut slow_ops = Vec::new();

    //TODO Worked around the Reference issue by splitting into scalar variables. Need to find out the right way to do
    let duration: i64 = opt.duration as i64;
    let mut binary = false;
    match opt.binary {
        None => {}
        Some(_) => binary = true,
    };
    let txt_len = opt.text_size;
    let depth = opt.nest_depth;
    let num_fields = opt.num_fields;
    let mut sequence = run_id_start;
    while elapsed_seconds < duration {
        let op = &op_weight[dist.sample(&mut rng)].0;
        let op_start_time = chrono::Utc::now();
        match op {
            MDBInsert => {
                sequence += 1;
                collection
                    .insert_one(
                        mongo_util::create_doc(
                            num_fields, depth, txt_len, binary, process_id, sequence,
                        ),
                        None,
                    )
                    .await?;
            }
            MDBQuery => {
                let filter = doc! { "_id": format!("w-{}-seq-{}", process_id, sequence)};
                let qdoc = collection.find_one(filter, None).await?;
                match qdoc {
                    Some(ref qdoc) => {
                        //TODO Do something
                    }
                    None => {
                        //TODO Do something
                    }
                }
            }
            MDBUpdate => {
                //TODO Do something
            }
        }
        let op_end_time = chrono::Utc::now();
        let op_time = op_end_time - op_start_time;
        if op_time.num_milliseconds() > 50 {
            slow_ops.push((op, op_time));
        }
        elapsed_seconds = chrono::Utc::now().timestamp() - start_time.timestamp();
    }

    if slow_ops.len() > 0 {
        info!("{} slow ops found", slow_ops.len());
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
