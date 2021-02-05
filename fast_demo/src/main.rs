use chrono::format::StrftimeItems;
use chrono::{NaiveDate, NaiveDateTime, Local};

struct BaseModel {
    id: u64,
    name: String,
}


struct Person {
    age: u32,
    school: String,
    baseModel: BaseModel,
}

impl Person {

    fn new(){

    }
}


fn main() {}
