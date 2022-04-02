 
#[derive(Debug, Clone)]
pub struct PathFilter {
    pub paths: Vec<String>
}

impl PathFilter {
    pub fn add(&mut self, path: String) -> &mut PathFilter {
        self.paths.push(path);
        return self;
    }

    pub fn new() -> PathFilter {
        let vec = Vec::<String>::new();
        PathFilter { paths: vec }
    }
}

#[test]
pub fn test() {
    let mut t = PathFilter::new();
    let x = t.add("abc".to_string()).add("bcd".to_owned());
    println!("{:?}", t);
}

