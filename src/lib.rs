use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

/// `TaskListGrammar` is a parser for task lists using a custom grammar defined in `grammar.pest`.
/// It supports parsing tasks with fields: priority, status, date, and description.
#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct TaskListGrammar;

/// `TaskParseError` is an enumeration of possible errors that may occur during task parsing.
#[derive(Error, Debug)]
pub enum TaskParseError {
    /// Error indicating an invalid year format.
    ///
    /// For example, if the year does not consist of 4 digits.
    #[error("Invalid year format: {0}")]
    InvalidYear(String),

    /// Error indicating an invalid priority format.
    ///
    /// The priority must consist of 1 to 3 exclamation marks (`!`).
    #[error("Invalid priority format: expected '!' between 1 to 3 times, got '{0}'")]
    InvalidPriority(String),

    /// Generic parsing error.
    ///
    /// Used for parsing errors that do not fit specific categories.
    #[error("Parsing error: {0}")]
    ParseError(String),
}

/// Parses a year from a string and returns it as `u32`.
///
/// # Grammar Rule
/// ```pest
/// year = { ASCII_DIGIT{4} }
/// ```
///
/// # Example
/// ```rust
/// use task_list::parse_year;
/// assert_eq!(parse_year("2024").unwrap(), 2024);
/// assert!(parse_year("20").is_err());
/// ```
///
/// # Errors
/// - `InvalidYear` if the string does not consist of 4 digits.
/// - `ParseError` if the parser fails to match the rule.
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

/// Parses the priority from a string and returns the priority level (1 to 3).
///
/// # Grammar Rule
/// ```pest
/// priority = { "!"{1,3} }
/// ```
///
/// # Example
/// ```rust
/// use task_list::parse_priority;
/// assert_eq!(parse_priority("!!").unwrap(), 2);
/// assert!(parse_priority("!!!!").is_err());
/// ```
///
/// # Errors
/// - `InvalidPriority` if the priority is not between 1 and 3 exclamation marks.
pub fn parse_priority(priority: &str) -> Result<u8, TaskParseError> {
    if !(1..=3).contains(&priority.len()) || !priority.chars().all(|c| c == '!') {
        return Err(TaskParseError::InvalidPriority(priority.to_string()));
    }

    Ok(priority.len() as u8)
}
