use git2::Repository;

fn main() {
    let repo_root = std::env::args().nth(1).unwrap_or("../".to_string());
    let repo = Repository::open(repo_root.as_str()).expect("cloud not repository");
    println!("{},{:?}", repo.path().display(), repo.state());
}