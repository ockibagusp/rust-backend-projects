mod file;
mod help;
mod list;
mod mark;
mod task;

use mockall::*;

use crate::list::list::{ListManager, ListManagerTrait};
use crate::mark::mark::{Mark, MarkTrait};
use crate::task::task_manager::{TaskManager, TaskManagerTrait};
use core::result::Result;
use std::io::{Error, ErrorKind};
// => add(...); v

struct Main {
    task_manager: TaskManager,
    mark: Mark,
    list_manager: ListManager,
}

#[automock]
trait MainTrait {
    fn new(file_name: &'static str) -> Self;
    fn command_of_task_cli(&mut self, args: &Vec<String>) -> Result<String, Error>;
    fn run(&mut self, args: &Vec<String>) -> Result<String, Error>;
}

impl MainTrait for Main {
    fn new(file_name: &'static str) -> Self {
        Main {
            task_manager: TaskManager::new(file_name),
            mark: Mark::new(file_name),
            list_manager: ListManager::new(file_name),
        }
    }

    /*
        Task Manager Operations
    */
    fn command_of_task_cli(&mut self, args: &Vec<String>) -> Result<String, Error> {
        let accept_args = &args[1];
        // $ task-cli add <task>
        if accept_args == "add" {
            if args.len() == 2 || args.len() > 3 {
                return Err(Error::new(ErrorKind::ConnectionRefused, help::help_add()));
            }
            // 2
            let input_args = &args[2];
            let task_result = self.task_manager.add(input_args);
            if let Err(e) = &task_result {
                return Err(Error::new(ErrorKind::ConnectionAborted, e.to_string()));
            }
            let task = task_result.unwrap();
            let task_manager_str: Vec<String> = vec![
                "Add tast:".to_string(),
                "------------------".to_string(),
                format!("ID: {}", task.id),
                format!("----- Description: {}", task.description),
                format!("----- Status: {}", task.status),
                format!("----- Created At: {:?}", task.created_at),
                format!("----- Updated At: {:?}", task.updated_at),
            ];

            return Ok(task_manager_str.join("\n"));
        }

        // $ task-cli update <id> <task>
        if accept_args == "update" {
            if args.len() == 3 || args.len() > 4 {
                return Err(Error::new(
                    ErrorKind::ConnectionRefused,
                    help::help_update(),
                ));
            }
            // 2
            let input_args = &args[2];
            let id: i32 = match input_args.parse() {
                Ok(num) => num,
                Err(e) => {
                    return Err(Error::new(
                        ErrorKind::ConnectionRefused,
                        format!("Error updating task: {}", e),
                    ));
                }
            };

            // 3
            let input_args = &args[3];
            let task_result = self.task_manager.update_description(id, input_args);
            let Ok(task) = task_result else {
                return Err(Error::new(
                    ErrorKind::ConnectionAborted,
                    format!("Error updating task: {}", task_result.unwrap_err()),
                ));
            };

            let task_manager_str: Vec<String> = vec![
                String::from("Update tast:"),
                String::from("------------------"),
                format!("ID: {}", task.id),
                format!("----- Description: {}", task.description),
                format!("----- Status: {}", task.status),
                format!("----- Created At: {:?}", task.created_at),
                format!("----- Updated At: {:?}", task.updated_at),
            ];

            return Ok(task_manager_str.join("\n"));
        }

        // $ task-cli delete <id>
        if accept_args == "delete" {
            if args.len() == 2 || args.len() > 3 {
                return Err(Error::new(
                    ErrorKind::ConnectionRefused,
                    help::help_delete(),
                ));
            }

            // 2
            let input_args = &args[2];
            let id: i32 = match input_args.parse() {
                Ok(num) => num,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::ConnectionRefused,
                        "Error deleting task: ID must be a number",
                    ));
                }
            };

            let delete_task = self.task_manager.delete(id);
            if let Err(e) = delete_task {
                return Err(Error::new(
                    ErrorKind::ConnectionAborted,
                    format!("Error deleting task: {}", e),
                ));
            }

            return Ok(String::from("Delete task success"));
        }

        return Err(Error::new(ErrorKind::ConnectionAborted, help::help_all()));
    }

    fn run(&mut self, args: &Vec<String>) -> Result<String, Error> {
        if args.len() == 1 || (args.len() == 2 && &args[1] == "help") {
            return Err(Error::new(ErrorKind::ConnectionRefused, help::help_all()));
        }

        let accept_args = &args[1];

        /*
            Task Operations
        */
        if accept_args == "add" || accept_args == "update" || accept_args == "delete" {
            match self.command_of_task_cli(args) {
                Ok(data) => return Ok(data),
                Err(e) => return Err(e),
            }
        }

        /*
            List Task Operations
        */
        // -------------------------------
        // $ task-cli list
        if accept_args == "list" {
            // func => tasks::list(); v
            //         ^^^^^
            //      => add(...); v
            // func => list(); x
            if args.len() == 2 {
                let list = self.list_manager.index();

                let mut list_str: Vec<String> =
                    vec!["Lists:".to_string(), "------------------".to_string()];
                if list.is_empty() {
                    list_str.push("No lists found.".to_string());
                } else {
                    for task in list {
                        list_str.push(format!("ID: {}", task.id));
                        list_str.push(format!("----- Description: {}", task.description));
                        list_str.push(format!("----- Status: {}", task.status));
                        list_str.push(format!("----- Created At: {:?}", task.created_at));
                        list_str.push(format!("----- Updated At: {:?}", task.updated_at));
                    }
                    list_str.push(String::from("++++++++++++++++++"));
                }

                return Ok(list_str.join("\n"));
            }

            return Err(Error::new(ErrorKind::ConnectionRefused, help::help_list()));
        }
        // -------------------------------

        return Ok(help::help_all());
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut main = Main::new("tasks.json");
    let result = main.run(&args);
    match result {
        Ok(data) => {
            println!("{}", data);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

mod tests {
    use crate::{MainTrait, MockMainTrait};
    use mockall::predicate::*;
    use std::{io::Error, vec};

    #[test]
    fn test_mock_run_no_args() {
        let args: Vec<String> = vec![String::from("task-cli")];

        let mut mock = MockMainTrait::default();

        mock.expect_run().with(eq(args.clone())).returning(|_| {
            Err(Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "Usage:...$ task-cli <accept> <input>",
            ))
        });
        let result = mock.run(&args);
        assert!(result.is_err());
        let err = result.unwrap_err();

        assert_eq!(err.kind(), std::io::ErrorKind::ConnectionRefused);
        let output_err = err.to_string();
        assert!(output_err.contains("Usage:"));
        assert!(output_err.contains("$ task-cli <accept> <input>"));
    }

    #[test]
    fn test_mock_run_task_add_should_fail() {
        struct TestCase {
            name: &'static str,
            args: Vec<String>,
            error_kind: std::io::ErrorKind,
        }

        let test_cases = vec![
            TestCase {
                name: "fail 1: no description provided",
                args: vec![String::from("task-cli"), String::from("add")],
                error_kind: std::io::ErrorKind::ConnectionRefused,
            },
            TestCase {
                name: "fail 1: too many arguments",
                args: vec![
                    String::from("task-cli"),
                    String::from("add"),
                    String::from("Task One"),
                    String::from("Extra Arg"),
                ],
                error_kind: std::io::ErrorKind::ConnectionRefused,
            },
            TestCase {
                name: "fail 2: too many description provided",
                args: vec![
                    String::from("task-cli"),
                    String::from("add"),
                    String::from("Task One Two Three Four Five"),
                ],
                error_kind: std::io::ErrorKind::ConnectionAborted,
            },
        ];

        for test_case in test_cases {
            let mut mock = MockMainTrait::default();
            mock.expect_command_of_task_cli()
                .with(always())
                .returning(move |_| {
                    Err(Error::new(
                        test_case.error_kind,
                        "Usage:...$ task-cli add <task>",
                    ))
                });
            let output = mock.command_of_task_cli(&test_case.args).unwrap_err();
            assert!(output.kind() == test_case.error_kind);
            assert!(
                output
                    .to_string()
                    .contains("Usage:...$ task-cli add <task>"),
                "{} failed",
                test_case.name
            );
        }
    }

    #[test]
    fn test_mock_run_task_add_should_success() {
        let args: Vec<String> = vec![
            String::from("task-cli"),
            String::from("add"),
            String::from("Test task"),
        ];

        let mut mock = MockMainTrait::default();
        mock.expect_command_of_task_cli()
            .with(always())
            .returning(|_| Ok(String::from("Add task success")));
        let output = mock.command_of_task_cli(&args).unwrap();
        assert_eq!(output, "Add task success");
    }

    #[test]
    fn test_mock_run_task_update_should_fail() {
        struct TestCase {
            name: &'static str,
            args: Vec<String>,
            error_kind: std::io::ErrorKind,
        }

        let test_cases = vec![
            TestCase {
                name: "fail 1: no description provided",
                args: vec![String::from("task-cli"), String::from("update")],
                error_kind: std::io::ErrorKind::ConnectionRefused,
            },
            TestCase {
                name: "fail 1: too many arguments",
                args: vec![
                    String::from("task-cli"),
                    String::from("update"),
                    String::from("1"),
                    String::from("foo bar"),
                    // too many args
                    String::from("extra-arg"),
                ],
                error_kind: std::io::ErrorKind::ConnectionRefused,
            },
            TestCase {
                name: "fail 2: Id not a number",
                args: vec![
                    String::from("task-cli"),
                    String::from("update"),
                    // id should be number
                    String::from("i"),
                    String::from("foo bar"),
                ],
                error_kind: std::io::ErrorKind::ConnectionRefused,
            },
            TestCase {
                name: "fail 3: Id not found",
                args: vec![
                    String::from("task-cli"),
                    String::from("update"),
                    // id should not be found
                    String::from("2"),
                    String::from("foo bar"),
                ],
                error_kind: std::io::ErrorKind::ConnectionAborted,
            },
        ];

        for test_case in test_cases {
            let mut mock = MockMainTrait::default();
            mock.expect_command_of_task_cli()
                .with(always())
                .returning(move |_| {
                    Err(Error::new(
                        test_case.error_kind,
                        "Usage:...$ task-cli update <id> <task>",
                    ))
                });
            let output = mock.command_of_task_cli(&test_case.args).unwrap_err();
            assert!(output.kind() == test_case.error_kind);
            assert!(
                output
                    .to_string()
                    .contains("Usage:...$ task-cli update <id> <task>"),
                "{} failed",
                test_case.name
            );
        }
    }

    #[test]
    fn test_mock_run_task_update_should_success() {
        let args: Vec<String> = vec![
            String::from("task-cli"),
            String::from("update"),
            String::from("1"),
            String::from("foo bar baz"),
        ];

        let mut mock = MockMainTrait::default();
        mock.expect_command_of_task_cli()
            .with(always())
            .returning(|_| Ok(String::from("Update task success")));
        let output = mock.command_of_task_cli(&args).unwrap();
        assert_eq!(output, "Update task success");
    }

    #[test]
    fn test_mock_run_task_delete_should_fail() {
        struct TestCase {
            name: &'static str,
            args: Vec<String>,
            error_kind: std::io::ErrorKind,
        }

        let test_cases = vec![
            TestCase {
                name: "fail 1: no description provided",
                args: vec![String::from("task-cli"), String::from("delete")],
                error_kind: std::io::ErrorKind::ConnectionRefused,
            },
            TestCase {
                name: "fail 1: too many arguments",
                args: vec![
                    String::from("task-cli"),
                    String::from("delete"),
                    String::from("1"),
                    // too many args
                    String::from("extra-arg"),
                ],
                error_kind: std::io::ErrorKind::ConnectionRefused,
            },
            TestCase {
                name: "fail 2: ID must be a number",
                args: vec![
                    String::from("task-cli"),
                    String::from("delete"),
                    // id should be number
                    String::from("i"),
                ],
                error_kind: std::io::ErrorKind::ConnectionRefused,
            },
            TestCase {
                name: "fail 3: ID not found",
                args: vec![
                    String::from("task-cli"),
                    String::from("delete"),
                    // id should not be found
                    String::from("2"),
                ],
                error_kind: std::io::ErrorKind::ConnectionAborted,
            },
        ];

        for test_case in test_cases {
            let mut mock = MockMainTrait::default();
            mock.expect_command_of_task_cli()
                .with(always())
                .returning(|_| {
                    Err(Error::new(
                        std::io::ErrorKind::ConnectionRefused,
                        "Usage:...$ task-cli update <id> <task>",
                    ))
                });
            let output = mock.command_of_task_cli(&test_case.args).unwrap_err();
            assert!(output.kind() == std::io::ErrorKind::ConnectionRefused);
            assert!(
                output
                    .to_string()
                    .contains("Usage:...$ task-cli update <id> <task>"),
                "{} failed",
                test_case.name
            );
        }
    }

    #[test]
    fn test_mock_run_task_delete_should_success() {
        let args: Vec<String> = vec![
            String::from("task-cli"),
            String::from("delete"),
            String::from("1"),
        ];

        let mut mock = MockMainTrait::default();
        mock.expect_command_of_task_cli()
            .with(always())
            .returning(|_| Ok(String::from("Delete task success")));
        let output = mock.command_of_task_cli(&args).unwrap();
        assert_eq!(output, "Delete task success");
    }
}
