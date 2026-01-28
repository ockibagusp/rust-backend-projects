mod tests {
    use crate::cmd::cmd::{CommandTrait, MockCommandTrait};
    use mockall::predicate::*;
    use std::{io::Error, vec};

    #[test]
    fn test_mock_run_no_args() {
        let args: Vec<String> = vec![String::from("task-cli")];

        let mut mock = MockCommandTrait::default();

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
            let mut mock = MockCommandTrait::default();
            mock.expect_task_cli().with(always()).returning(move |_| {
                Err(Error::new(
                    test_case.error_kind,
                    "Usage:...$ task-cli add <task>",
                ))
            });
            let output = mock.task_cli(&test_case.args).unwrap_err();
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

        let mut mock = MockCommandTrait::default();
        mock.expect_task_cli()
            .with(always())
            .returning(|_| Ok(String::from("Add task success")));
        let output = mock.task_cli(&args).unwrap();
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
            let mut mock = MockCommandTrait::default();
            mock.expect_task_cli().with(always()).returning(move |_| {
                Err(Error::new(
                    test_case.error_kind,
                    "Usage:...$ task-cli update <id> <task>",
                ))
            });
            let output = mock.task_cli(&test_case.args).unwrap_err();
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

        let mut mock = MockCommandTrait::default();
        mock.expect_task_cli()
            .with(always())
            .returning(|_| Ok(String::from("Update task success")));
        let output = mock.task_cli(&args).unwrap();
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
            let mut mock = MockCommandTrait::default();
            mock.expect_task_cli().with(always()).returning(move |_| {
                Err(Error::new(
                    test_case.error_kind,
                    "Usage:...$ task-cli update <id> <task>",
                ))
            });
            let output = mock.task_cli(&test_case.args).unwrap_err();
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

        let mut mock = MockCommandTrait::default();
        mock.expect_task_cli()
            .with(always())
            .returning(|_| Ok(String::from("Delete task success")));
        let output = mock.task_cli(&args).unwrap();
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
            let mut mock = MockCommandTrait::default();
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
        let mut mock = MockCommandTrait::default();
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
        let mut mock = MockCommandTrait::default();
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
        let mut mock = MockCommandTrait::default();
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
        let mut mock = MockCommandTrait::default();
        mock.expect_run()
            .with(always())
            .returning(|_| Ok(String::from("List done task success")));
        let output = mock.run(&args).unwrap();
        assert_eq!(output, "List done task success");
    }
}
