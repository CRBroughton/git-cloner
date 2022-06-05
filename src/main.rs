use std::{process::Command as OSCommand};
use clap::{Command, arg};

fn main() {
    let matches = Command::new("Git Cloner")
        .version("0.0.1")
        .author("Craig R Broughton <CRBroughton@posteo.uk")
        .about("Clones git repositories")
        .arg_required_else_help(true)
        .subcommand(Command::new("repo")
            .about("Your chosen repository")
            .arg(arg!([REPO_PATH])))
        .subcommand(Command::new("init")
            .about("Initialises a git repository"))
        .get_matches();

        match matches.subcommand() {
            Some(("init", _sub_matches)) => init_repo(),
            Some(("repo", sub_matches)) => clone_repo(sub_matches.value_of("REPO_PATH").unwrap().to_string()),
            _ => println!("You failed to enter any known command..."),
        }
}


fn init_repo() {
    let output = OSCommand::new("git")
        .arg("init")
        .output()
        .expect("failed to initialised repository");

        println!("{:?}", String::from_utf8_lossy(&output.stderr).trim_end());
}

fn clone_repo(repo_link: String) {
    let output = OSCommand::new("git")
    .arg("clone")
    .arg(repo_link)
    .output()
    .expect("failed to initialised repository");

    println!("{:?}", String::from_utf8_lossy(&output.stderr).trim_end());
}