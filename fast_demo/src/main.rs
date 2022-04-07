use config_struct::StructOptions;

fn main() {
    let result = config_struct::create_struct("./setting.toml", "src/config.rs", &StructOptions::default());
    println!("{:?}",result);
}


