use bson::{Document, Uuid};
use bson::spec::BinarySubtype::Generic;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{TypeDate, TypeInt, TypeText};

#[derive(Serialize, Deserialize)]
struct MongoDoc {
    document: Document,
    num_fields: u16,
    depth: u8,
    txt_len: usize,
    binary: bool,
}

pub fn create_doc(num_fields: u16, depth: u8, txt_len: usize, binary: bool, process_id: usize, run_id_start: usize) -> Document {
    let mut mongo_doc = MongoDoc {
        document: Document::new(),
        num_fields,
        depth,
        txt_len,
        binary,
    };
    mongo_doc.add_id(process_id, run_id_start);
    mongo_doc.add_fields();
    mongo_doc.add_binary();
    mongo_doc.document
}

fn create_string(len: usize) -> String {
    let random_text = lipsum::lipsum(len);
    random_text.to_string()
}

trait Create {
    fn add_fields(&mut self);
    fn add_id(&mut self, process_id: usize, run_id_start: usize);
    fn add_binary(&mut self);
    fn field_type(&self, field_num: u16) -> FieldTypes;
}

impl Create for MongoDoc {
    fn add_fields(&mut self) {
        let mut field_num: u16 = 0;
        while field_num < self.num_fields {
            let f_type = self.field_type(field_num);
            let i: u32 = rand::thread_rng().gen();
            let s = create_string(self.txt_len);
            match f_type {
                TypeInt => self.document.insert(format!("{}{}", "fld", field_num), i),
                TypeDate => self.document.insert(format!("{}{}", "fld", field_num), chrono::Utc::now() ),
                _ => self.document.insert(format!("{}{}", "fld", field_num), s),
            };
            field_num += 1;
        }
    }

    fn add_id(&mut self, process_id: usize, sequence: usize) {
        // self.document.insert("_id", Uuid::new());
        self.document.insert("_id", format!("w-{}-seq-{}", process_id, sequence));
    }

    fn add_binary(&mut self) {
        if self.binary {
            let bin = bson::Binary{
                subtype: Generic,
                //TODO - binary with value
                bytes: Vec::new()
            };
            self.document.insert(format!("{}{}", "fld", "_binary"), bin);
        }
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

pub enum FieldTypes {
    TypeInt,
    TypeDate,
    TypeText,
}

pub(crate) enum Ops {
    MDBInsert,
    MDBQuery,
    MDBUpdate,
}