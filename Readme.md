# Task List Parser in Rust

- **Crates.io**: https://crates.io/crates/task_list_parser
- **Docs**: https://docs.rs/task_list_parser/latest/task_list_parser/

## Overview
This project is a task list parser written in Rust using the `pest` parsing library. It can read and interpret structured task lists, with fields like priority, completion status, start date, optional tags, time estimate, and description.

## Grammar Rules

- **Priority**: Indicates importance using `!`, `!!`, or `!!!`.
- **Status**: `[ ]` for incomplete, `[x]` for completed.
- **Date**: `{YYYY-MM-DD}` format.
- **Tags**: Hashtags for categorization (e.g., `#urgent`).
- **Time Estimate**: Time estimate in hours (`h`) or minutes (`m`).

### Example Input

- `!! [ ] {2024-11-05} #shopping 1h Buy groceries`
- `!!! [x] {2024-12-01} #work 2h Finish Rust project`
- `! [ ] {2025-01-15} #learning 5h Prepare presentation`

### Running the Parser

```bash
cargo run -- --file tasks.txt
