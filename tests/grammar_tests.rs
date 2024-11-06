use anyhow::{anyhow, Ok};
use pest::Parser;
use task_list::{Rule, TaskListGrammar};

#[test]
fn test_valid_dates() -> anyhow::Result<()> {
    let valid_dates = vec![
        "{2024-01-01}",
        "{1999-12-31}",
        "{2023-06-15}",
        "{2024-02-29}",
    ];

    for date in valid_dates {
        let result = TaskListGrammar::parse(Rule::date, date)?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(result.as_str(), date);
    }

    Ok(())
}

#[test]
fn test_invalid_dates() -> anyhow::Result<()> {
    let invalid_dates = vec![
        "{2024-13-01}",
        "{2024-00-10}",
        "{2023-11-32}",
        "{2023-02-99}",
        "{202a-01-01}",
        "{2024-1-01}",
        "{2024-11-}",
        "{-2024-11-05}",
        "{}"
    ];

    for date in invalid_dates {
        let result = TaskListGrammar::parse(Rule::date, date);
        assert!(result.is_err(), "Expected invalid date: {}", date);
    }

    Ok(())
}