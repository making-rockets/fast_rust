#[derive(Debug, Clone)]
pub struct PathFilter {
    pub paths: Vec<String>,
}

impl PathFilter {
    pub fn _add(&mut self, path: String) -> &mut PathFilter {
        self.paths.push(path);
        self
    }

    pub fn _new() -> PathFilter {
        let vec = Vec::<String>::new();
        PathFilter { paths: vec }
    }
}
