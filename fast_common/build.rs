use config_struct::StructOptions;

fn main() {
    println!("cargo:rerun-if-changed=./setting.toml");
    config_struct::create_struct("./setting.toml", "./src/config/toml_config.rs", &StructOptions::default());
}