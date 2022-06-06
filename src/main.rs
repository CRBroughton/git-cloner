use clap::{arg, Command};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{process::Command as OSCommand, fs::File, io::{Read, Write}, error::Error};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct JsonData {
    cloner: String,
    vue: String,
}

fn main() {
    let matches = Command::new("Git Cloner")
        .version("0.1.0")
        .author("Craig R Broughton <CRBroughton@posteo.uk")
        .about("Clones git repositories")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("repo")
                .about("Your chosen repository")
                .arg(arg!([REPO_PATH])),
        )
        .subcommand(Command::new("cloner").about("Clones this repository"))
        .subcommand(Command::new("vue").about("Clones the Vue repository"))
        .subcommand(Command::new("init").about("Initialises a git repository"))
        .get_matches();

    
    let json_data = parse_json_file();

    if json_data.is_err() {
        match create_repos_file() {
            Err(err) => { println!("{}", err)}
            _ => println!("I don't know what's happening")
        }
    }

    match matches.subcommand() {
        Some(("init", _sub_matches)) => init_repo(),
        Some(("cloner", _sub_matches)) => {
            if json_data.is_ok() {
                clone_repo(json_data.unwrap().cloner);
            } else {
                println!("Creating JSON File...");
            }
        }
        Some(("vue", _sub_matches)) => {
            if json_data.is_ok() {
                clone_repo(json_data.unwrap().vue);
            } else {
                println!("Creating JSON File...");
            }
        }
        Some(("repo", sub_matches)) => {
            if sub_matches.value_of("REPO_PATH").is_some() {
                clone_repo(sub_matches.value_of("REPO_PATH").unwrap().to_string())
            } else {
                println!("You failed to provide a valid path...")
            }
        }
        _ => println!("You failed to enter any known command..."),
    }
}

fn create_repos_file()-> Result<(), Box<dyn Error>> {
    let mut file = File::create("repos.json")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

fn parse_json_file() -> Result<JsonData, Box<dyn Error>> {
    let mut file = File::open("repos.json")?;
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
 
    let foo: JsonData = serde_json::from_str(&buff).unwrap();

    Ok(foo)
}

fn init_repo() {
    let output = OSCommand::new("git")
        .arg("init")
        .output()
        .expect("failed to initialised repository");

    println!("{:?}", String::from_utf8_lossy(&output.stdout).trim_end());
}

fn clone_repo(repo_link: String) {
    let output = OSCommand::new("git")
        .arg("clone")
        .arg(repo_link)
        .output()
        .expect("failed to initialised repository");

    println!("{:?}", String::from_utf8_lossy(&output.stderr).trim_end());
}
