use dotenvy::dotenv;
use rr_common::get_project_root;
use std::fs;
use wizard::errors::WizardError;

#[tokio::main]
async fn main() -> Result<(), WizardError> {
    dotenv().ok();

    // println!("current dir = {:?}", std::env::current_dir());
    let project_root = get_project_root().expect("Cannot find project root");
    let main_file = project_root.join("main.yaml");
    let content = fs::read_to_string(main_file).expect("Unable to read main.yaml file");

    let config = wizard::config::parse_main_config(&content)?;

    println!("config {:?}", config);

    Ok(())
}
