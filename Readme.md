# Task List Parser in Rust

## Overview
This project is a task list parser written in Rust using the `pest` parsing library. The parser is designed to read and interpret structured task lists, with each line containing the task's priority, completion status, start date, and description. This tool is ideal for applications involving task tracking or project management, where structured input is parsed into meaningful data for further processing or display.

## Parsing Process
The task list parser follows a defined grammar to extract details from task descriptions with the following structure:

- **Priority**: Indicates the importance of the task using one (`!`), two (`!!`), or three (`!!!`) exclamation marks.
- **Status**: Shows if the task is completed or not using `[ ]` for incomplete and `[x]` for complete.
- **Start Date**: A mandatory date field in `YYYY-MM-DD` format.
- **Description**: A detailed description of the task, which can contain any text up to the end of the line.

### Example Input

! [ ] 2024-11-05 Buy groceries 
!! [x] 2024-10-30 Finish Rust parser project 
!!! [ ] 2024-11-10 Prepare for the meeting on Monday