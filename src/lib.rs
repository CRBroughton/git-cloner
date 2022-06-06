use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    process::Command as OSCommand,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonData {
    pub cloner: String,
    pub vue: String,
}

pub fn create_repos_file() -> Result<(), Box<dyn Error>> {
    let mut file = File::create("repos.json")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

pub fn parse_json_file() -> Result<JsonData, Box<dyn Error>> {
    let mut file = File::open("repos.json")?;
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let foo: JsonData = serde_json::from_str(&buff).unwrap();

    Ok(foo)
}

pub fn init_repo() {
    let output = OSCommand::new("git")
        .arg("init")
        .output()
        .expect("failed to initialised repository");

    println!("{:?}", String::from_utf8_lossy(&output.stdout).trim_end());
}

pub fn clone_repo(repo_link: String) {
    let output = OSCommand::new("git")
        .arg("clone")
        .arg(repo_link)
        .output()
        .expect("failed to initialised repository");

    println!("{:?}", String::from_utf8_lossy(&output.stderr).trim_end());
}
