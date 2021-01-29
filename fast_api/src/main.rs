use std::fmt::Error;

fn main() {
    let version = parse_version(&[20]);
    match version {
        Ok(v) => println!("version: {:?}", v),
        Err(e) => println!("其实我就是想测试一下这个: {:?}", e),
    }

    let reft = &format!("{}{}", "tee".to_string(), "abc".to_string());

    let t = reft;

    println!("{}", reft);
}


#[derive(Debug)]
enum Version { Version1, Version2 }

fn parse_version(header: &[i32]) -> Result<Version, &'static str> {
     for x in header {
     let re =      match x {
            0..=10 =>   Ok(Version::Version2),
            _ =>   Err("meiyou a")
        };
      return re;
    };

    return Err("zhaobudao");
}

