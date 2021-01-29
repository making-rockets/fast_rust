#[macro_use]
extern crate fast_common;

use fast_common::utils::crypt_util;

use std::fmt::Error;

fn main() {
    let x = r#"{"persons":[{"name" : "Joe","age" : 12}]}"#;
    let result = crypt_util::encrypt(&x);
    println!("{:?}", result);

    let result1 = crypt_util::decrypt_string(result.unwrap().as_ref());
    println!("{:?}", result1);
    let stu = Student{ person: Person { name: "tt".to_string(), age: 10 }, school: "ac".to_string() };
    println!("{:?}",stu.get_person());
}

//java的继承
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}
#[derive(Debug)]
struct Student {
    person: Person,
    school: String,
}

impl Person {
    fn get_person(&self) -> Self {
        Person { name: self.name.clone(), age: 0 }
    }
}

impl Student {
    fn get_person(&self) ->Person {
        self.person.get_person()
    }
}


#[derive(Debug)]
enum Version {
    Version1,
    Version2,
}

fn parse_version(header: &[i32]) -> Result<Version, &'static str> {
    for x in header {
        let re = match x {
            0..=10 => Ok(Version::Version2),
            _ => Err("meiyou a"),
        };
        return re;
    }

    return Err("zhaobudao");
}
