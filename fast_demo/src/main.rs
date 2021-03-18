use std::ops::{Deref, DerefMut};

struct Person {
    name: String
}

impl Person {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn say(&self) {
        println!("我是一个人，我叫{}", self.name);
    }
}

struct Student {
    p: Person,
    role: String,
}

impl Student {
    fn new() ->Student{
        Student{ p: Person { name: "person".to_string() }, role: "11".to_string() }
    }
    fn set_role(&mut self ,role:&str){
        self.role = role.to_string();
    }
}

impl Deref for Student{
    type Target = Person;

    fn deref(&self) -> &Self::Target {
        &self.p
    }
}

impl DerefMut for Student {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.p
    }
}


struct A {
    data: i32
}

struct B {
    a: A
}

impl B {
    fn set_data(&mut self, data: i32) {
        unsafe {
            (self.a).data = data
        };
    }
}


fn main() {
    //let mut student = Student::new();
    //student.set_role("办证");
    //student.say()
}