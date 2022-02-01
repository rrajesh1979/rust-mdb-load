use crate::{mongo_util, stats_reporter, Opt, INSERTS, QUERIES, UPDATES};
use bson::{doc, Document};
use mongodb::Client;
use std::error::Error;

use crate::mongo_util::create_string;
use crate::{Insert, Query, Update};
use rand::distributions::{Distribution, WeightedIndex};
use rand::{thread_rng, Rng};

/// Initialize MongoDB database and collection
#[tokio::main]
pub async fn mongodb_init(opt: Opt) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = Client::with_uri_str(opt.conn).await?;
    let namespace = opt.namespace;
    let (db, coll) = parse_namespace(&namespace);
    let database = client.database(&*db);
    let collection = database.collection::<Document>(&*coll);
    let _db_init_result = collection.drop(None).await?;
    info!("Clean-up of {}.{} completed!", &*db, &*coll);

    Ok(())
}

#[tokio::main]
pub async fn mongodb_load_gen(
    opt: Opt,
    process_id: usize,
    run_id_start: usize,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    //TODO Can we keep a common connection across threads?
    let client = Client::with_uri_str(opt.conn).await?;
    let namespace = opt.namespace;
    let (db, coll) = parse_namespace(&namespace);
    let database = client.database(&*db);
    let collection = database.collection(&*coll);
    let mut elapsed_seconds: i64 = 0;
    let start_time = chrono::Utc::now();

    let op_weight = [
        (Insert, opt.inserts),
        (Query, opt.queries),
        (Update, opt.updates),
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
    while elapsed_seconds <= duration {
        let op = &op_weight[dist.sample(&mut rng)].0;
        let op_type: &str;
        let op_start_time = chrono::Utc::now();
        match op {
            Insert => {
                sequence += 1;
                let _insert_result = collection
                    .insert_one(
                        mongo_util::create_doc(
                            num_fields, depth, txt_len, binary, process_id, sequence,
                        ),
                        None,
                    )
                    .await?;

                stats_reporter::record_ops_done(&INSERTS, 1);
                op_type = &INSERTS;
            }
            Query => {
                let filter = doc! { "_id": format!("w-{}-seq-{}", process_id, sequence)};
                let _qdoc = collection.find_one(filter, None).await?;
                if let Some(ref _qdoc) = _qdoc {
                    stats_reporter::record_ops_done(&QUERIES, 1);
                }
                op_type = &QUERIES;
            }
            Update => {
                //TODO Implement
                let updated_int: u32 = rand::thread_rng().gen();
                // info!("sequence {}", sequence);
                let update_seq: usize = rand::thread_rng().gen_range(0..sequence);
                // info!("update_seq {}", update_seq);
                let updated_text = create_string(txt_len);
                let updated_date = chrono::Utc::now();
                let filter = doc! { "_id": format!("w-{}-seq-{}", process_id, update_seq)};
                let update_doc = doc! {
                    "$set": {
                        "fld0": updated_int,
                        "fld2": updated_text,
                        "fld1": updated_date
                    }
                };
                let update_result = collection.update_one(filter, update_doc, None).await?;
                if update_result.modified_count > 0 {
                    stats_reporter::record_ops_done(&UPDATES, update_result.modified_count as i32);
                }
                op_type = &UPDATES;
            }
        }
        let op_end_time = chrono::Utc::now();
        let op_time = op_end_time - op_start_time;
        if op_time.num_milliseconds() > 50 {
            stats_reporter::record_slow_ops(op_type, &op_time);
            slow_ops.push((op, op_time));
        }
        elapsed_seconds = chrono::Utc::now().timestamp() - start_time.timestamp();
    }

    if !slow_ops.is_empty() {
        info!("{} slow ops found", slow_ops.len());
    }

    Ok(())
}

fn parse_namespace(ns: &str) -> (String, String) {
    let namespace: Vec<&str> = ns.split('.').collect();
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
