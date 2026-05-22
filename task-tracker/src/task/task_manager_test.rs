use std::fs;
use std::io::Error;

// use mockall::mock;
use crate::task::task::TaskTrait;
use crate::task::task_manager::{MockTaskManagerTrait, TaskManager, TaskManagerTrait, *};
use crate::task::task_test;
use mockall::predicate::*;

use std::io::ErrorKind::InvalidInput;

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
    let m = TaskManager::new();
    assert!(Some(m.file).is_some());
    assert_eq!(m.list, vec![]);
    fs::remove_file(file_name).unwrap();
}

const TASK_DESC: &str = "test buy one";
const UPDATED_DESC: &str = "test updated description";
const ERR_DESC_EMPTY: &str = "Error { code: \"TASK_MANAGER\", kind: InvalidInput, message: \"DESCRIPTION is too short(min. 2 chars) or too long(max. 50 chars)\" }";
const ERR_NOT_FOUND: &str =
    "Error { code: \"TASK_MANAGER\", kind: InvalidInput, message: \"ID is not found\" }";

#[test]
fn test_add_in_get_next_add_task_should_return_task_with_error() {
    let list = vec![];

    // description with 1 character
    let result = get_next_task_of_add(&list, "f");
    assert!(result.is_err());
    let err_task = result.unwrap_err().to_string();
    assert_eq!(err_task, ERR_DESC_EMPTY);

    // description with more than 50 characters
    let result = get_next_task_of_add(
        &list,
        "foo bar baz qux quux corge grault garply waldo fred plugh xyzzy thud",
    );
    assert!(result.is_err());
    let err_task = result.unwrap_err().to_string();
    assert_eq!(err_task, ERR_DESC_EMPTY);
}

#[test]
fn test_add_in_get_next_add_task_should_return_task_with_correct_id() {
    let list = vec![];
    let result = get_next_task_of_add(&list, TASK_DESC);
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, 1);
    assert_eq!(task.description, TASK_DESC.to_string());
    assert_eq!(task.status, "todo");
}

#[test]
fn test_mock_add_should_success() {
    let mut mock = MockTaskManagerTrait::default();

    let add_task = task_test::setup_task(2, TASK_DESC);
    mock.expect_add()
        .with(eq(TASK_DESC))
        .returning(move |_| Ok(add_task.clone()));
    let result = mock.add(TASK_DESC);
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, 2);
    assert_eq!(task.description, TASK_DESC.to_string());
    assert_eq!(task.status, "todo");
}

#[test]
fn test_find_by_id_should_return_task_with_error() {
    let list = vec![];
    let result = find_by_id(&list, -1);
    assert!(result.is_err());
    let err_task = result.unwrap_err().to_string();
    assert_eq!(err_task, ERR_NOT_FOUND);
}

#[test]
fn test_find_by_id_should_return_task_with_correct() {
    let list = vec![task_test::setup_task(1, TASK_DESC)];
    let result = find_by_id(&list, 1);
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, 1);
    assert_eq!(task.description, TASK_DESC.to_string());
    assert_eq!(task.status, "todo");
}

#[test]
fn test_mock_update_description_should_fail() {
    let mut mock = MockTaskManagerTrait::default();

    // is too short description
    mock.expect_update_description()
        .with(eq(1), eq("f"))
        .returning(|_, _| Err(Error::new(std::io::ErrorKind::InvalidInput, ERR_DESC_EMPTY)));
    let result = mock.update_description(1, "f");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), InvalidInput);
    assert_eq!(err.to_string(), ERR_DESC_EMPTY);
}

#[test]
fn test_mock_update_description_should_success() {
    let mut mock = MockTaskManagerTrait::default();

    // find by id before update
    mock.expect_update_description()
        .with(eq(1), eq(TASK_DESC))
        .returning(|_, _| Ok(task_test::setup_task_status(1, TASK_DESC, "todo")));
    let task_before = mock.update_description(1, TASK_DESC);

    assert!(task_before.is_ok());
    let task_before = task_before.unwrap();
    assert_eq!(task_before.description, TASK_DESC);
    assert!(task_before.status == "todo");

    // update description success and find by id after update
    let mut mock = MockTaskManagerTrait::default();
    mock.expect_update_description()
        .with(eq(1), eq(UPDATED_DESC))
        .returning(|_, _| Ok(task_test::setup_task_status(1, UPDATED_DESC, "todo")));
    let task_after = mock.update_description(1, UPDATED_DESC);
    assert!(task_after.is_ok());
    let task_after = task_after.unwrap();
    assert_eq!(task_after.description, UPDATED_DESC);
    assert!(task_after.status == "todo");
}

#[test]
fn test_update_in_find_by_id_should_with_error() {
    let err = task_test::setup_task(-1, "f");
    let result = err.is_validation();
    assert_eq!(result.unwrap_err(), "ID is negative");
}

#[test]
fn test_update_in_find_by_id_should_with_correct() {
    let empy = task_test::setup_task(1, "foo");
    let result = empy.is_validation();
    assert_eq!(result.unwrap(), ());
}

const DESCRIPTION: i32 = 0;
#[test]
fn test_update_in_is_not_similar_to_task_of_description_or_status_update_should_error() {
    let list = vec![task_test::setup_task(1, TASK_DESC)];
    let mut task = task_test::setup_task(1, TASK_DESC);
    let invalid =
        is_valid_to_task_of_description_or_status_update(&list, 1, &mut task, DESCRIPTION);
    assert!(invalid)
}

#[test]
fn test_update_in_is_not_similar_to_task_of_description_or_status_update_should_correct() {
    let list = vec![task_test::setup_task(1, TASK_DESC)];
    let mut task = task_test::setup_task(1, UPDATED_DESC);
    let invalid =
        is_valid_to_task_of_description_or_status_update(&list, 1, &mut task, DESCRIPTION);
    assert_eq!(invalid, false)
}

#[test]
fn test_mock_update_should_fail() {
    let mut mock = MockTaskManagerTrait::default();

    let mut update_task_fail = task_test::setup_task(1, "f");
    // is empty description
    mock.expect_updates()
        .with(eq(1), eq(update_task_fail.clone()), eq(DESCRIPTION))
        .returning(|_, _, _| Err(Error::new(InvalidInput, ERR_DESC_EMPTY)));
    let result = mock.updates(1, &mut update_task_fail, DESCRIPTION);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), ERR_DESC_EMPTY);
}

#[test]
fn test_mock_update_should_success() {
    let mut mock: MockTaskManagerTrait = MockTaskManagerTrait::default();

    // is empty description
    let mut update_task = task_test::setup_task_status(1, TASK_DESC, "done");
    let value = update_task.clone();
    mock.expect_updates()
        .with(eq(1), eq(update_task.clone()), eq(DESCRIPTION))
        .returning(move |_, _, _| Ok(value.clone()));
    let result = mock.updates(1, &mut update_task, DESCRIPTION);
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, 1);
    assert_eq!(task.description, TASK_DESC);
    assert_eq!(task.status, "done");
}

// !???
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
