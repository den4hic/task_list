use anyhow::{anyhow, Ok};
use pest::Parser;
use task_list_parser::{Rule, TaskListGrammar};

#[cfg(test)]
mod tests {
    use super::*;

    mod date_tests {
        use std::any;

        use super::*;

        #[test]
        fn valid_date() -> anyhow::Result<()> {
            let pair = TaskListGrammar::parse(Rule::date, "{2023-09-15}")?
                .next()
                .unwrap();
            assert_eq!(pair.as_str(), "{2023-09-15}");
            Ok(())
        }

        #[test]
        fn valid_year() -> anyhow::Result<()> {
            let pair = TaskListGrammar::parse(Rule::year, "2022")?.next().unwrap();
            assert_eq!(pair.as_str(), "2022");
            Ok(())
        }

        #[test]
        fn invalid_year() {
            assert!(
                TaskListGrammar::parse(Rule::year, "123").is_err(),
                "Year with incorrect format should fail"
            );
        }

        #[test]
        fn invalid_date_format() {
            assert!(
                TaskListGrammar::parse(Rule::date, "{2023/09/15}").is_err(),
                "Date with incorrect format should fail"
            );
        }

        #[test]
        fn out_of_range_date() {
            assert!(
                TaskListGrammar::parse(Rule::date, "{2023-13-15}").is_err(),
                "Date with invalid month should fail"
            );
            assert!(
                TaskListGrammar::parse(Rule::date, "{2023-09-32}").is_err(),
                "Date with invalid day should fail"
            );
        }
    }

    mod priority_tests {
        use super::*;

        #[test]
        fn valid_priority() -> anyhow::Result<()> {
            let pair = TaskListGrammar::parse(Rule::priority, "!!")?
                .next()
                .unwrap();
            assert_eq!(pair.as_str(), "!!");
            Ok(())
        }

        #[test]
        fn invalid_priority() -> anyhow::Result<()> {
            let pair = TaskListGrammar::parse(Rule::priority, "!!!!")?
                .next()
                .unwrap();

            assert!(pair.as_str().len() <= 3);

            Ok(())
        }
    }

    mod title_tests {
        use super::*;

        #[test]
        fn valid_title() -> anyhow::Result<()> {
            let pair = TaskListGrammar::parse(Rule::description, "Meeting with team")?
                .next()
                .unwrap();
            assert_eq!(pair.as_str(), "Meeting with team");
            Ok(())
        }

        #[test]
        fn empty_title() {
            assert!(
                TaskListGrammar::parse(Rule::description, "").is_err(),
                "Empty title should fail"
            );
        }
    }

    mod tags_tests {
        use super::*;

        #[test]
        fn valid_tags() -> anyhow::Result<()> {
            let pair = TaskListGrammar::parse(Rule::tags, "#urgent #work")?
                .next()
                .unwrap();
            assert_eq!(pair.as_str(), "#urgent #work");
            Ok(())
        }

        #[test]
        fn invalid_tags_format() {
            assert!(TaskListGrammar::parse(Rule::tags, "urgent work").is_err());
        }
    }

    mod comment_tests {
        use super::*;

        #[test]
        fn valid_comment() -> anyhow::Result<()> {
            let pair = TaskListGrammar::parse(Rule::comment, "// This is a comment")?
                .next()
                .unwrap();
            assert_eq!(pair.as_str(), "// This is a comment");
            Ok(())
        }

        #[test]
        fn invalid_comment() {
            assert!(TaskListGrammar::parse(Rule::comment, "Comment").is_err());
        }
    }

    #[test]
    fn test_complete_task() -> anyhow::Result<()> {
        let task = "!! [ ] {2024-10-30} #work 2h Finish Rust parser project";
        let pair = TaskListGrammar::parse(Rule::task, task)?.next().unwrap();
        assert_eq!(pair.as_str(), task);
        Ok(())
    }

    #[test]
    fn test_task_with_missing_tags() {
        let task = "adwadasd";
        assert!(TaskListGrammar::parse(Rule::task, task).is_err());
    }
}
