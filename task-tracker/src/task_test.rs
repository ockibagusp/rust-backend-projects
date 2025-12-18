use std::fs;
use std::io::{Error, ErrorKind};

use crate::files;
use crate::task::TaskManagerTrait;
use crate::task::{MockTaskManagerTrait, MockTaskTrait, TaskTrait};
// use mockall::mock;
use crate::task::TaskManager;
use chrono::DateTime;
use mockall::predicate::*;

/*
    Task
*/
#[test]
fn test_task_trait_should_fail() {
    let mut mock = MockTaskTrait::new();
    let invalid_input = ErrorKind::InvalidInput;

    mock.expect_is_validation()
        .returning(move || Err(Error::new(invalid_input, "error")));
    assert_eq!(mock.is_validation().is_err(), true);
}

#[test]
fn test_task_trait_should_success() {
    let mut mock = MockTaskTrait::default();
    mock.expect_is_validation().returning(|| Ok(()));
    assert_eq!(mock.is_validation().unwrap(), ());
}

/*
    TaskManager
*/
// mock! {
//     pub TaskManager {
//         pub file: crate::files::File,
//         pub list: Vec<Task>,
//         pub next_id: i32,
//     }

//     impl TaskManagerTrait for TaskManager {
//         fn new(file_name: &'static str) -> Self;
//         fn get_next_id(&self) -> i32;
//         fn list(&self) -> Vec<crate::task::Task>;
//         fn add(&mut self, input: &str) -> Result<crate::task::Task, Error>;
//     }
// }
#[test]
fn test_task_manager_trait_new() {
    let file_name = "test-task-cli.json";
    let m = TaskManager::new(file_name);
    assert!(Some(m.file).is_some());
    assert_eq!(m.list, vec![]);
    fs::remove_file(file_name).unwrap();
}

#[test]
fn test_mock_list() {
    // // Setup Mock for new, so no function
    // MockTaskManagerTrait::expect_new()
    //     .with(mockall::predicate::eq("test-task-cli"))
    //     .returning(|name| MockTaskManagerTrait {
    //         name,
    //         ..Default::default()
    //     });

    // let mut mock = MockTaskManagerTrait::new("test-task-cli");

    /*
     * empty list
     */
    let mut mock = MockTaskManagerTrait::default();
    mock.expect_list().times(1).return_once(|| vec![]);
    assert_eq!(mock.list(), vec![]);

    /*
     * one task in list
     */
    // test with one task
    let task = files::tests::setup_task(1, "test one");
    let task_one = task.clone();

    mock.expect_list()
        .times(1)
        .return_once(move || vec![task.clone()]);
    assert_eq!(mock.list(), vec![task_one]);
}

#[test]
fn test_mock_add_should_fail() {
    let created_at = DateTime::parse_from_str("1970-01-01 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
        .unwrap()
        .into();

    // negative id
    let mut _add_task_fail = crate::task::Task {
        id: -1,
        description: "test buy one".to_string(),
        status: crate::task::VALID_STATUSES[0].to_string(),
        created_at: created_at,
        updated_at: created_at,
    };

    let mut mock = MockTaskManagerTrait::default();

    mock.expect_add().with(eq("test buy one")).returning(|_| {
        Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "error: `id` is negative",
        ))
    });
    let result = mock.add("test buy one");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), "error: `id` is negative");

    // is empty description
    mock.expect_add().with(eq("")).returning(|_| {
        Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "error: `description` is empty or too long",
        ))
    });
    let result = mock.add("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), "error: `description` is empty or too long");

    // is too long description
    let long_description = "a".repeat(30);
    mock.expect_add()
        .with(eq(long_description.clone()))
        .returning(|_| {
            Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "error: `description` is empty or too long",
            ))
        });
    let result = mock.add(&long_description);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), "error: `description` is empty or too long");

    // is invalid status
    // Note: Since status is not passed in add function, this case is just for demonstration or is never used.
}

#[test]
fn test_mock_add_should_success() {
    let created_at = DateTime::parse_from_str("1970-01-01 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
        .unwrap()
        .into();

    const TASK_DESC: &str = "test buy one";
    let mut _add_task = crate::task::Task {
        id: 2,
        description: TASK_DESC.to_string(),
        status: crate::task::VALID_STATUSES[0].to_string(),
        created_at: created_at,
        updated_at: created_at,
    };

    let mut mock = MockTaskManagerTrait::default();

    mock.expect_add()
        .with(eq(TASK_DESC))
        .returning(move |_| Ok(_add_task.clone()));
    let result = mock.add(TASK_DESC);
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, 2);
    assert_eq!(task.description, TASK_DESC.to_string());
}

#[test]
fn test_mock_update_should_fail() {
    let created_at = DateTime::parse_from_str("1970-01-01 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
        .unwrap()
        .into();
    let updated_at = DateTime::parse_from_str("1970-01-01 00:00:01 +00:00", "%Y-%m-%d %H:%M:%S %z")
        .unwrap()
        .into();

    let mut _update_task_fail = crate::task::Task {
        id: 1,
        description: "".to_string(),
        status: crate::task::VALID_STATUSES[0].to_string(),
        created_at: created_at,
        updated_at: updated_at,
    };

    let mut mock = MockTaskManagerTrait::default();

    // is empty description
    mock.expect_update()
        .with(eq(1), eq(_update_task_fail.clone()))
        .returning(|_, _| {
            Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "error: `description` is empty or too long",
            ))
        });
    let result = mock.update(1, &mut _update_task_fail);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), "error: `description` is empty or too long");

    // is too long description
    _update_task_fail.description = "a".repeat(30);
    // // Note: Since updatad_at is not passed in update function, this case is just for demonstration or is never used.
    // _update_task_fail.updated_at =
    //     DateTime::parse_from_str("1970-01-01 00:00:02 +00:00", "%Y-%m-%d %H:%M:%S %z")
    //         .unwrap()
    //         .into();
    mock.expect_update()
        .with(eq(4), eq(_update_task_fail.clone()))
        .returning(|_, _| {
            Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "error: `description` is empty or too long",
            ))
        });
    let result = mock.update(4, &mut _update_task_fail);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), "error: `description` is empty or too long");
}

#[test]
fn test_mock_update_should_success() {
    let created_at = DateTime::parse_from_str("1970-01-01 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
        .unwrap()
        .into();
    let updated_at = DateTime::parse_from_str("1970-01-01 00:10:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
        .unwrap()
        .into();

    let mut _update_task_fail = crate::task::Task {
        id: 1,
        description: "".to_string(),
        status: crate::task::VALID_STATUSES[0].to_string(),
        created_at: created_at,
        updated_at: updated_at,
    };

    let mut mock = MockTaskManagerTrait::default();

    // is empty description
    mock.expect_update()
        .with(eq(1), eq(_update_task_fail.clone()))
        .returning(|_, _| {
            Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "error: `description` is empty or too long",
            ))
        });
    let result = mock.update(1, &mut _update_task_fail);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), "error: `description` is empty or too long");

    // is too long description
    _update_task_fail.description = "a".repeat(30);
    // // Note: Since updatad_at is not passed in update function, this case is just for demonstration or is never used.
    // _update_task_fail.updated_at =
    //     DateTime::parse_from_str("1970-01-01 00:00:02 +00:00", "%Y-%m-%d %H:%M:%S %z")
    //         .unwrap()
    //         .into();
    mock.expect_update()
        .with(eq(4), eq(_update_task_fail.clone()))
        .returning(|_, _| {
            Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "error: `description` is empty or too long",
            ))
        });
    let result = mock.update(4, &mut _update_task_fail);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), "error: `description` is empty or too long");
}

#[test]
fn test_mock_delete_should_fail() {
    let mut mock = MockTaskManagerTrait::default();

    // negative id
    mock.expect_delete().with(eq(-1)).returning(|_| {
        Err(Error::new(
            std::io::ErrorKind::NotFound,
            "error: `id` is not found",
        ))
    });
    let result = mock.delete(-1);
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    assert_eq!(err.to_string(), "error: `id` is not found");
}

#[test]
fn test_mock_delete_should_success() {
    let mut mock = MockTaskManagerTrait::default();

    // negative id
    mock.expect_delete().with(eq(1)).returning(|_| Ok(()));
    let result = mock.delete(1);
    assert_eq!(result.unwrap(), ());
}
