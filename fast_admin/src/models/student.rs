use std::fmt::format;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub class: String,
    pub DOB: String,
    pub parent_name: String,
    pub mobile: String,
    pub address: String,
}

pub async fn get_students() -> anyhow::Result<Vec<Student>> {
    let mut studetns = Vec::new();
    for i in 0..10 {
        let student = Student {
            id: i.to_string(),
            name: format!("{}{}", i, "名称"),
            class: format!("{}{}", i, "班级"),
            DOB: format!("{}{}", i, "DOB"),
            parent_name: format!("{}{}", i, "父级名称"),
            mobile: format!("{}{}", i, "手机号"),
            address: format!("{}{}", i, "地址"),
        };

        studetns.push(student);
    }

    Ok(studetns)
}

pub async fn get_student(student_id: i64) -> anyhow::Result<Student> {
    Ok(Student {
        id: student_id.to_string(),
        name: "两拨1".to_owned(),
        class: "两拨2".to_owned(),
        DOB: "两拨3".to_owned(),
        parent_name: "两拨4".to_owned(),
        mobile: "两拨5".to_owned(),
        address: "两拨6".to_owned(),
    })
}
