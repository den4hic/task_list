use anyhow::Result;
use clap::{Arg, Command};
use pest::Parser;
use std::fs;
use task_list::*;

fn main() -> Result<()> {
    let matches = Command::new("TaskListParser")
        .version("1.0.0")
        .author("Denys Davydov d.davydov@ukma.edu.ua")
        .about("Parses a custom task list format")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Path to the task list file to parse")
                .required(false),
        )
        .arg(
            Arg::new("show-credits")
                .long("show-credits")
                .help("Displays credits information")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("show-credits") {
        println!("TaskListParser by Your Name");
        return Ok(());
    }

    if let Some(file_path) = matches.get_one::<String>("file") {
        let content = fs::read_to_string(file_path)?;
        let parsed = TaskListGrammar::parse(Rule::task_list, &content)?;
        println!("Parsed successfully: {:?}", parsed);
    } else {
        println!("No file provided. Use --help for more information.");
    }

    Ok(())
}
