use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct TaskListGrammar;

#[derive(Error, Debug)]
pub enum TaskParseError {
    #[error("Invalid year format: {0}")]
    InvalidYear(String),

    #[error("Invalid priority format: expected '!' between 1 to 3 times, got '{0}'")]
    InvalidPriority(String),

    #[error("Parsing error: {0}")]
    ParseError(String),
}

pub fn parse_year(year: &str) -> Result<u32, TaskParseError> {
    let pair = TaskListGrammar::parse(Rule::year, year)
        .map_err(|_| TaskParseError::InvalidYear(year.to_string()))?
        .next()
        .ok_or_else(|| TaskParseError::ParseError("Failed to parse year".to_string()))?;

    let year_value = pair
        .as_str()
        .parse::<u32>()
        .map_err(|_| TaskParseError::InvalidYear(year.to_string()))?;

    Ok(year_value)
}

pub fn parse_priority(priority: &str) -> Result<u8, TaskParseError> {
    if !(1..=3).contains(&priority.len()) || !priority.chars().all(|c| c == '!') {
        return Err(TaskParseError::InvalidPriority(priority.to_string()));
    }

    Ok(priority.len() as u8)
}
