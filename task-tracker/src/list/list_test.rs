use crate::list::list::{ListManagerTrait, MockListManagerTrait};
use crate::task::task::VALID_STATUSES;
use crate::task::task_test::{setup_task, setup_task_status};

#[test]
// func. `index` is the same as (equals) `list`
fn test_mock_list() {
    /*
     * empty list
     */
    let mut mock = MockListManagerTrait::default();
    mock.expect_index().times(1).return_once(|| vec![]);
    assert_eq!(mock.index(), vec![]);

    /*
     * one task in list
     */
    // test with one task
    let task = setup_task(1, "test one");
    let task_one = task.clone();

    mock.expect_index()
        .times(1)
        .return_once(move || vec![task.clone()]);
    assert_eq!(mock.index(), vec![task_one]);
}

#[test]
fn test_mock_todo() {
    const TASK_DESC: &str = "test buy one";
    let mut _task = setup_task_status(1, TASK_DESC, "todo");

    let mut mock = MockListManagerTrait::default();
    mock.expect_todo()
        .with()
        .returning(move || vec![_task.clone()]);
    let result = mock.todo();
    assert_eq!(result[0].id, 1);
    assert_eq!(result[0].description, TASK_DESC);
    assert_eq!(result[0].status, VALID_STATUSES[0]);
}

#[test]
fn test_mock_in_progress() {
    const TASK_DESC: &str = "test buy two";
    let mut _task = setup_task_status(2, TASK_DESC, "in-progress");

    let mut mock = MockListManagerTrait::default();
    mock.expect_in_progress()
        .with()
        .returning(move || vec![_task.clone()]);
    let result = mock.in_progress();
    assert_eq!(result[0].id, 2);
    assert_eq!(result[0].description, TASK_DESC);
    assert_eq!(result[0].status, VALID_STATUSES[1]);
}

#[test]
fn test_mock_done() {
    const TASK_DESC: &str = "test buy three";
    let mut _task = setup_task_status(4, TASK_DESC, "done");

    let mut mock = MockListManagerTrait::default();
    mock.expect_done()
        .with()
        .returning(move || vec![_task.clone()]);
    let result = mock.done();
    assert_eq!(result[0].id, 4);
    assert_eq!(result[0].description, TASK_DESC);
    assert_eq!(result[0].status, crate::task::task::VALID_STATUSES[2]);
}
