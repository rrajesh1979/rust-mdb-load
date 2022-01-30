use std::error::Error;
use mongodb::Client;
use crate::{mongo_util, Opt};

pub async fn mongodb_load_gen(opt: Opt) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = Client::with_uri_str(opt.conn).await?;
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
        collection.insert_one(mongo_util::create_doc(num_fields, depth, txt_len, binary), None).await?;
        elapsed_seconds = chrono::Utc::now().timestamp() - start_time.timestamp();
    }
    Ok(())
}
