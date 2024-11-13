use anyhow::Result;
use std::fs;
use pest::Parser;
use task_list::*;

fn main() -> Result<()> {
    let content = fs::read_to_string("tasks.txt")?;
    let parsed = TaskListGrammar::parse(Rule::task_list, &content)?;
    println!("Parsed successfully: {:?}", parsed);
    Ok(())
}
