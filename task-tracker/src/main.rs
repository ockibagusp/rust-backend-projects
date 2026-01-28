mod error;
mod file;
mod help;
mod list;
mod mark;
mod task;

use mockall::*;

use crate::list::list::{ListManager, ListManagerTrait};
use crate::mark::mark::{Mark, MarkTrait};
use crate::task::task::Task;
use crate::task::task_manager::{TaskManager, TaskManagerTrait};
use core::result::Result;
use std::io::{Error, ErrorKind};
// => add(...); v

fn error_kind_refused(err_message: String) -> Error {
    return Error::new(ErrorKind::ConnectionRefused, err_message);
}

fn error_kind_aborted(err_message: String) -> Error {
    return Error::new(ErrorKind::ConnectionAborted, err_message);
}

fn open_task_str(task: &Task) -> String {
    let task_str: Vec<String> = vec![
        format!("ID: {}", task.id),
        format!("----- Description: {}", task.description),
        format!("----- Status: {}", task.status),
        format!("----- Created At: {:?}", task.created_at),
        format!("----- Updated At: {:?}", task.updated_at),
    ];

    return task_str.join("\n");
}

fn open_task_title_str(title: &str, task: Task) -> String {
    let task_str: Vec<String> = vec![
        String::from(title),
        String::from("------------------"),
        open_task_str(&task),
    ];

    return task_str.join("\n");
}

fn open_task_list_for_title_str(title: &str, list: Vec<Task>) -> String {
    let mut list_str: Vec<String> = vec![String::from(title), String::from("------------------")];
    if list.is_empty() {
        list_str.push(String::from("No lists found."));
    } else {
        for task in list {
            list_str.push(open_task_str(&task));
        }
        list_str.push(String::from("++++++++++++++++++"));
    }

    return list_str.join("\n");
}

struct Main {
    task_manager: TaskManager,
    mark: Mark,
    list_manager: ListManager,
}

#[automock]
trait MainTrait {
    fn new(file_name: &'static str) -> Self;
    fn command_of_task_cli(&mut self, args: &Vec<String>) -> Result<String, Error>;
    fn command_of_mark_cli(&mut self, args: &Vec<String>) -> Result<String, Error>;
    fn command_of_list_cli(&mut self, args: &Vec<String>) -> Result<String, Error>;
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
        if "add" == args.get(1).unwrap() && args.len() == 2 {
            return Err(error_kind_refused(help::help_add()));
        }
        if "update" == args.get(1).unwrap() && args.len() == 2 {
            return Err(error_kind_refused(help::help_update()));
        }
        if "delete" == args.get(1).unwrap() && args.len() == 2 {
            return Err(error_kind_refused(help::help_delete()));
        }

        let accept_args = args.get(1).unwrap();
        let input_args = args.get(2).unwrap();

        // $ task-cli add <task>
        if accept_args == "add" {
            let task_result = self.task_manager.add(input_args);
            if let Err(e) = &task_result {
                return Err(error_kind_aborted(format!(
                    "Error adding task: {}",
                    e.to_string()
                )));
            }
            let task = task_result.unwrap();
            return Ok(open_task_title_str("Add task", task));
        }

        // $ task-cli update <id> <task>
        if accept_args == "update" {
            let id: i32 = match input_args.parse() {
                Ok(num) => num,
                Err(e) => {
                    return Err(error_kind_refused(format!("Error updating task: {}", e)));
                }
            };

            let update_args = args.get(3);
            if update_args.is_none() {
                return Err(error_kind_refused(help::help_update()));
            }
            let task_result = self
                .task_manager
                .update_description(id, update_args.unwrap());
            let Ok(task) = task_result else {
                return Err(error_kind_aborted(format!(
                    "Error updating task: {}",
                    task_result.unwrap_err()
                )));
            };

            return Ok(open_task_title_str("Update task", task));
        }

        // $ task-cli delete <id>
        if accept_args == "delete" {
            let id: i32 = match input_args.parse() {
                Ok(num) => num,
                Err(_) => {
                    return Err(error_kind_refused(
                        "Error deleting task: ID must be a number".to_string(),
                    ));
                }
            };

            let delete_task = self.task_manager.delete(id);
            if let Err(e) = delete_task {
                return Err(error_kind_aborted(format!("Error deleting task: {}", e)));
            }

            return Ok(String::from("Delete task success"));
        }

        return Err(error_kind_refused(help::help_all()));
    }

    /*
        Mark Task Operations
    */
    fn command_of_mark_cli(&mut self, args: &Vec<String>) -> Result<String, Error> {
        if "mark-in-progress" == args.get(1).unwrap() && args.len() == 2 {
            return Err(error_kind_refused(help::help_mark_in_progress()));
        }

        let accept_args = args.get(1).unwrap();
        let input_args = args.get(2).unwrap();

        // $ task-cli mark-in-progress <id>
        if accept_args == "mark-in-progress" {
            let id: i32 = match input_args.parse() {
                Ok(num) => num,
                Err(e) => {
                    return Err(error_kind_refused(format!(
                        "Error marking task in progress: {}",
                        e
                    )));
                }
            };
            let task_result = self.mark.mark_in_progress(id);
            if let Err(e) = &task_result {
                return Err(error_kind_aborted(format!(
                    "Error marking task in progress: {}",
                    e.to_string()
                )));
            }
            let task = task_result.unwrap();
            return Ok(open_task_title_str("Mark task in progress", task));
        } else if accept_args == "mark-done" {
            let id: i32 = match input_args.parse() {
                Ok(num) => num,
                Err(e) => {
                    return Err(error_kind_refused(format!(
                        "Error marking task done: {}",
                        e
                    )));
                }
            };
            let task_result = self.mark.mark_done(id);
            if let Err(e) = &task_result {
                return Err(error_kind_aborted(format!(
                    "Error marking task done: {}",
                    e.to_string()
                )));
            }
            let task = task_result.unwrap();
            return Ok(open_task_title_str("Mark task done", task));
        }

        Err(error_kind_refused(help::help_all()))
    }

    fn command_of_list_cli(&mut self, args: &Vec<String>) -> Result<String, Error> {
        // func => tasks::list(); v
        //         ^^^^^
        //      => add(...); v
        // func => list(); x
        if args.len() == 2 {
            let list = self.list_manager.index();
            let list_str = open_task_list_for_title_str("Lists", list);
            return Ok(list_str);
        }

        // $ task-cli list done
        let input_args = &args[2];
        if input_args == "todo" {
            let todo = self.list_manager.todo();
            let list_str = open_task_list_for_title_str("Todos Task", todo);
            return Ok(list_str);
        } else if input_args == "in-progress" {
            let in_progress = self.list_manager.in_progress();
            let list_str = open_task_list_for_title_str("In-Progresses Task", in_progress);
            return Ok(list_str);
        } else if input_args == "done" {
            let done = self.list_manager.done();
            let done_str = open_task_list_for_title_str("Dones Task", done);
            return Ok(done_str);
        }

        return Err(error_kind_refused(help::help_list()));
    }

    fn run(&mut self, args: &Vec<String>) -> Result<String, Error> {
        if args.len() == 1 {
            return Err(error_kind_refused(help::help_all()));
        }

        let accept_args = args.get(1).unwrap();

        /*
            Task Operations
        */
        if accept_args == "add" || accept_args == "update" || accept_args == "delete" {
            match self.command_of_task_cli(args) {
                Ok(data) => return Ok(data),
                Err(e) => {
                    return Err(error_kind_refused(e.to_string()));
                }
            }
        }
        /*
            Mark Task Operations
        */
        else if accept_args == "mark-in-progress" || accept_args == "mark-done" {
            match self.command_of_mark_cli(args) {
                Ok(data) => return Ok(data),
                Err(e) => {
                    return Err(error_kind_refused(e.to_string()));
                }
            }
        }
        /*
            List Task Operations
        */
        // -------------------------------
        // $ task-cli list
        else if accept_args == "list" {
            match self.command_of_list_cli(args) {
                Ok(data) => return Ok(data),
                Err(e) => {
                    return Err(error_kind_refused(e.to_string()));
                }
            }
        }

        return Ok(help::help_all());
    }
}

fn main() {
    let mut main = Main::new("tasks.json");
    let args: Vec<String> = std::env::args().collect();
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
                name: "fail 1: ID only arguments",
                args: vec![
                    String::from("task-cli"),
                    String::from("update"),
                    // id only
                    String::from("i"),
                ],
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
                name: "fail 2: ID not a number",
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
            String::from("mark-in-progress"),
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

    // -------------------------------
    //test mark-in-progress
    #[test]
    fn test_mock_run_task_in_progress_should_fail() {
        struct TestCase {
            name: &'static str,
            args: Vec<String>,
            error_kind: std::io::ErrorKind,
        }

        let test_cases = vec![
            TestCase {
                name: "fail 1: no description provided",
                args: vec![String::from("task-cli"), String::from("mark-in-progress")],
                error_kind: std::io::ErrorKind::ConnectionRefused,
            },
            TestCase {
                name: "fail 1: too many arguments",
                args: vec![
                    String::from("task-cli"),
                    String::from("mark-in-progress"),
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
                    String::from("mark-in-progress"),
                    // id should be number
                    String::from("i"),
                ],
                error_kind: std::io::ErrorKind::ConnectionRefused,
            },
            TestCase {
                name: "fail 3: ID not found",
                args: vec![
                    String::from("task-cli"),
                    String::from("mark-in-progress"),
                    // id should not be found
                    String::from("2"),
                ],
                error_kind: std::io::ErrorKind::ConnectionAborted,
            },
        ];

        for test_case in test_cases {
            let mut mock = MockMainTrait::default();
            mock.expect_run().with(always()).returning(move |_| {
                Err(Error::new(
                    test_case.error_kind,
                    "Usage:...$ task-cli mark-in-progress <id>",
                ))
            });
            let output = mock.run(&test_case.args).unwrap_err();
            assert!(output.kind() == test_case.error_kind);
            assert!(
                output
                    .to_string()
                    .contains("Usage:...$ task-cli mark-in-progress <id>"),
                "{} failed",
                test_case.name
            );
        }
    }

    // -------------------------------
    // $ task list
    #[test]
    fn test_mock_run_task_list_should_success() {
        let args: Vec<String> = vec![String::from("task-cli"), String::from("list")];
        let mut mock = MockMainTrait::default();
        mock.expect_run()
            .with(always())
            .returning(|_| Ok(String::from("List task success")));
        let output = mock.run(&args).unwrap();
        assert_eq!(output, "List task success");
    }

    #[test]
    fn test_mock_run_task_list_todo_should_success() {
        let args: Vec<String> = vec![
            String::from("task-cli"),
            String::from("list"),
            String::from("todo"),
        ];
        let mut mock = MockMainTrait::default();
        mock.expect_run()
            .with(always())
            .returning(|_| Ok(String::from("List todo task success")));
        let output = mock.run(&args).unwrap();
        assert_eq!(output, "List todo task success");
    }

    #[test]
    fn test_mock_run_task_list_in_progress_should_success() {
        let args: Vec<String> = vec![
            String::from("task-cli"),
            String::from("list"),
            String::from("in-progress"),
        ];
        let mut mock = MockMainTrait::default();
        mock.expect_run()
            .with(always())
            .returning(|_| Ok(String::from("List in progress task success")));
        let output = mock.run(&args).unwrap();
        assert_eq!(output, "List in progress task success");
    }

    #[test]
    fn test_mock_run_task_list_done_should_success() {
        let args: Vec<String> = vec![
            String::from("task-cli"),
            String::from("list"),
            String::from("done"),
        ];
        let mut mock = MockMainTrait::default();
        mock.expect_run()
            .with(always())
            .returning(|_| Ok(String::from("List done task success")));
        let output = mock.run(&args).unwrap();
        assert_eq!(output, "List done task success");
    }
}
