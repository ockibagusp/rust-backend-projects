use crate::task::TaskManager;
use crate::task::TaskManagerTrait;
use crate::task::{MockTaskManagerTrait, MockTaskTrait, Task, TaskTrait};
use crate::task_test;
use chrono::DateTime;
use chrono_tz::Africa::Tripoli;
use mockall::predicate::*;

/*
    Task
*/
#[test]
fn test_task_trait_fail() {
    let mut mock = MockTaskTrait::new();
    mock.expect_is_validation().returning(|| false);
    assert_eq!(mock.is_validation(), false);
}

#[test]
fn test_task_trait_success() {
    let mut mock = MockTaskTrait::new();
    mock.expect_is_validation().returning(|| true);
    assert_eq!(mock.is_validation(), true);
}

/*
    TaskManager
*/
#[test]
fn test_mock_list_fail() {
    let mut t = <TaskManager as TaskManagerTrait>::new("test-task-cli.json");
    let _ = t.list();
    // test

    // let task = Task {
    //     id: 1,
    //     description: String::from("buy milk"),
    //     status: TaskStatus::Todo,
    //     created_at: DateTime::parse_from_str(
    //         "2025-04-10 10:10:10.000000 +07:00",
    //         "%Y-%m-%d %H:%M:%S%.6f %z",
    //     )
    //     .unwrap()
    //     .into(),
    //     updated_at: DateTime::parse_from_str(
    //         "2025-04-10 10:10:10.000000 +07:00",
    //         "%Y-%m-%d %H:%M:%S%.6f %z",
    //     )
    //     .unwrap()
    //     .into(),
    // };

    // let m = task.clone();

    // let mut mock = MockTaskManagerTrait::new("test-task-cli.log");
    // mock.expect_list()
    //     .times(1)
    //     .returning(move || vec![task.clone()]);
    // assert_eq!(mock.list(), vec![m]);
}

// #[test]
// fn test_add_success() {
//     let created_at = DateTime::parse_from_str("1970-01-01 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
//         .unwrap()
//         .into();

//     let _add_task = Task {
//         id: 2,
//         description: "test".to_string(),
//         status: TaskStatus::Todo,
//         created_at: created_at,
//         updated_at: created_at,
//     };

//     let mut mock = MockTaskManagerTrait::new("test.json");
//     mock.expect_add()
//         .with(eq("test"))
//         .returning(|_| Ok(_add_task.clone()));
//     assert_eq!(mock.add("test"), Ok(_add_task));
// }

// #[test]
// fn test_mock_add_success() {
//     let mut mock = MockTaskTrait::new();
//     mock.expect_add()
//         .with(eq("test buy eggs 2"))
//         .times(1)
//         .returning(|x| get_task_add(2, x));
//     assert_eq!(
//         get_task_add(2, "test buy eggs 2"),
//         mock.add("test buy eggs 2")
//     );
// }

// // #[test]
// // fn test_update_success() {
// //     let test_task = get_task_update(3, "test buy 3 eggs");

// //     assert_eq!(test_task, update(3, "test buy 3 eggs"));
// // }

// #[test]
// fn test_delete_success() {
//     let test_task = delete(1);
//     assert_eq!(test_task, true);
// }

// #[test]
// fn test_delete_fail() {
//     let test_task = delete(2);
//     assert_eq!(test_task, false);

//     let test_task = delete(-1);
//     assert_eq!(test_task, false);
// }
