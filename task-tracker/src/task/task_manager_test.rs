use std::fs;
use std::io::Error;

// use mockall::mock;
use crate::task::task_manager::{MockTaskManagerTrait, TaskManager, TaskManagerTrait};
use mockall::predicate::*;

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

const TASK_DESC: &str = "test buy one";
const UPDATED_DESC: &str = "test updated description";
const ERR_ID_NEGATIVE: &str = "error: `id` is negative";
const ERR_DESC_EMPTY: &str = "error: `description` is empty or too long";
const ERR_NOT_FOUND: &str = "error: `id` is not found";

#[test]
fn test_mock_add_should_fail() {
    let mut mock = MockTaskManagerTrait::default();
    // negative id

    mock.expect_add().with(eq(TASK_DESC)).returning(|_| {
        Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            ERR_ID_NEGATIVE,
        ))
    });
    let result = mock.add(TASK_DESC);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), ERR_ID_NEGATIVE);

    // is empty description
    mock.expect_add()
        .with(eq(""))
        .returning(|_| Err(Error::new(std::io::ErrorKind::InvalidInput, ERR_DESC_EMPTY)));
    let result = mock.add("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), ERR_DESC_EMPTY);

    // is too long description
    let long_description = "a".repeat(30);
    mock.expect_add()
        .with(eq(long_description.clone()))
        .returning(|_| Err(Error::new(std::io::ErrorKind::InvalidInput, ERR_DESC_EMPTY)));
    let result = mock.add(&long_description);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), ERR_DESC_EMPTY);

    // is invalid status
    // Note: Since status is not passed in add function, this case is just for demonstration or is never used.
}

#[test]
fn test_mock_add_should_success() {
    let mut mock = MockTaskManagerTrait::default();

    let add_task = crate::task::task_test::setup_task(2, TASK_DESC);
    mock.expect_add()
        .with(eq(TASK_DESC))
        .returning(move |_| Ok(add_task.clone()));
    let result = mock.add(TASK_DESC);
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, 2);
    assert_eq!(task.description, TASK_DESC.to_string());
}

#[test]
fn test_mock_update_description_should_fail() {
    let mut mock = MockTaskManagerTrait::default();

    // is empty description
    mock.expect_update_description()
        .with(eq(1), eq(""))
        .returning(|_, _| Err(Error::new(std::io::ErrorKind::InvalidInput, ERR_DESC_EMPTY)));
    let result = mock.update_description(1, "");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), ERR_DESC_EMPTY);

    // is too long description
    let description_fail = "a".repeat(30);
    // // Note: Since updatad_at is not passed in update function, this case is just for demonstration or is never used.
    // _update_task_fail.updated_at =
    //     DateTime::parse_from_str("1970-01-01 00:00:02 +00:00", "%Y-%m-%d %H:%M:%S %z")
    //         .unwrap()
    //         .into();
    mock.expect_update_description()
        .with(eq(4), eq(description_fail.clone()))
        .returning(|_, _| Err(Error::new(std::io::ErrorKind::InvalidInput, ERR_DESC_EMPTY)));
    let result = mock.update_description(4, &description_fail);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), ERR_DESC_EMPTY);
}

#[test]
fn test_mock_update_description_should_success() {
    let mut mock = MockTaskManagerTrait::default();

    // find by id before update
    mock.expect_find_by_id().with(eq(1)).returning(|_| {
        Ok(crate::task::task_test::setup_task_status(
            1, TASK_DESC, "todo",
        ))
    });
    let task_before = mock.find_by_id(1);
    assert!(task_before.is_ok());
    assert!(task_before.unwrap().status == "todo");

    // update description success and find by id after update
    let mut mock = MockTaskManagerTrait::default();
    mock.expect_find_by_id().with(eq(1)).returning(|_| {
        Ok(crate::task::task_test::setup_task_status(
            1,
            UPDATED_DESC,
            "in-progress",
        ))
    });
    let task_after = mock.find_by_id(1);
    assert!(task_after.is_ok());
    assert!(task_after.unwrap().status == "in-progress");
}

#[test]
fn test_mock_update_should_fail() {
    let mut mock = MockTaskManagerTrait::default();

    let mut update_task_fail = crate::task::task_test::setup_task(1, "");
    // is empty description
    mock.expect_update()
        .with(eq(1), eq(update_task_fail.clone()))
        .returning(|_, _| Err(Error::new(std::io::ErrorKind::InvalidInput, ERR_DESC_EMPTY)));
    let result = mock.update(1, &mut update_task_fail);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), ERR_DESC_EMPTY);

    // is too long description
    update_task_fail.description = "a".repeat(30);
    // // Note: Since updatad_at is not passed in update function, this case is just for demonstration or is never used.
    // _update_task_fail.updated_at =
    //     DateTime::parse_from_str("1970-01-01 00:00:02 +00:00", "%Y-%m-%d %H:%M:%S %z")
    //         .unwrap()
    //         .into();
    mock.expect_update()
        .with(eq(4), eq(update_task_fail.clone()))
        .returning(|_, _| Err(Error::new(std::io::ErrorKind::InvalidInput, ERR_DESC_EMPTY)));
    let result = mock.update(4, &mut update_task_fail);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), ERR_DESC_EMPTY);
}

#[test]
fn test_mock_update_should_success() {
    let mut mock: MockTaskManagerTrait = MockTaskManagerTrait::default();

    // is empty description
    let mut update_task = crate::task::task_test::setup_task_status(1, TASK_DESC, "done");
    let value = update_task.clone();
    mock.expect_update()
        .with(eq(1), eq(update_task.clone()))
        .returning(move |_, _| Ok(value.clone()));
    let result = mock.update(1, &mut update_task);
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, 1);
    assert_eq!(task.description, TASK_DESC);
    assert_eq!(task.status, "done");
}

#[test]
fn test_mock_delete_should_fail() {
    let mut mock = MockTaskManagerTrait::default();
    // negative id
    mock.expect_delete()
        .with(eq(-1))
        .returning(|_| Err(Error::new(std::io::ErrorKind::NotFound, ERR_NOT_FOUND)));
    let result = mock.delete(-1);
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    assert_eq!(err.to_string(), ERR_NOT_FOUND);
}

#[test]
fn test_mock_delete_should_success() {
    let mut mock = MockTaskManagerTrait::default();

    // negative id
    mock.expect_delete().with(eq(1)).returning(|_| Ok(()));
    let result = mock.delete(1);
    assert_eq!(result.unwrap(), ());
}
