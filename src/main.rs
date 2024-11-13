use anyhow::Result;
use clap::{Arg, Command};
use pest::Parser;
use std::fs;
use task_list_parser::*;

fn main() -> Result<()> {
    let matches = Command::new("task_list_parser")
        .version("1.0.0")
        .author("Your Name <your.email@example.com>")
        .about("A CLI tool for parsing task lists with a custom grammar")
        .long_about(
            "task_list_parser is a command-line tool designed to parse task list files \
            written in a specific format. The format includes fields like priority, \
            status, date, and description.\n\n\
            Example task format:\n\
            ! [x] {2024-11-13} Complete the Rust parser project\n\
            !! [ ] {2024-12-01} Write comprehensive documentation"
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Specifies the path to the task list file to parse")
                .long_help(
                    "Provide the path to a file that contains the task list in a predefined format. \
                    Each task in the file should include priority (1-3 exclamation marks), \
                    status (completed or pending), date (YYYY-MM-DD), and a description."
                ),
        )
        .arg(
            Arg::new("show-credits")
                .long("show-credits")
                .help("Displays the credits for this tool")
                .long_help("Displays information about the author and contributors of the tool, including version details."),
        )
        .get_matches();


    if matches.contains_id("show-credits") {
        println!("Task List Parser by Your Name. Version 1.0.0.");
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
