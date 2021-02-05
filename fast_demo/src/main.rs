use chrono::format::StrftimeItems;
use chrono::{NaiveDate, NaiveDateTime, Local};
#[derive(Debug)]
struct BaseModel {
    id: u64,
    name: String,
}

#[derive(Debug)]
struct Person {
    age: u32,
    school: String,
    baseModel: BaseModel,
}

impl Person {

    fn new(age:u32,school:String,baseModel:BaseModel) ->Self{
        Self{
            age,
            school,
            baseModel
        }
    }

}


fn main() {
    let person = Person::new(1, "1".to_string(), BaseModel { id: 0, name: "".to_string() });
    println!("{:?}", person);
}
