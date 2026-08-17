use crate::{
    domain::{
        task::{Task, TaskStatus},
        task_test::{setup_task, setup_task_status},
    },
    infrastructure::{config, storages::storage::StorageTrait},
};

pub struct MockStorage;
impl StorageTrait for MockStorage {
    fn new(_config: &config::Config) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn find_by_list(&self) -> Vec<Task> {
        return vec![
            setup_task(1, "test description one"),
            setup_task_status(2, "test description two", TaskStatus::InProgress),
            setup_task_status(3, "test description two", TaskStatus::Done),
        ];
    }

    fn add(&self, _add_task: &Task) -> Vec<Task> {
        return vec![_add_task.clone()];
    }

    fn update(&self, _id: i32, _update_task: &Task) -> Vec<Task> {
        if _id != _update_task.id {
            panic!("ID does not match");
        }

        return vec![_update_task.clone()];
    }

    fn delete(&self, _id: i32) -> Vec<Task> {
        let tasks = &self.find_by_list();
        let mut task_vec = vec![];
        for task in tasks {
            if task.id != _id {
                task_vec.push(task.clone());
            }
        }
        task_vec
    }
}

// use crate::domain::task::{
//     task_manager::{MockTaskManagerTrait, TaskManagerTrait},
//     task_test,
// };
// use crate::domain::error::AppError::InvalidInput;
// use mockall::predicate::eq;
// use std::io::ErrorKind::InvalidInput;

// const FILE_NAME: &str = "FILE_TEST";

// #[test]
// fn test_mock_add_should_fail() {
//     let mut mock = MockTaskManagerTrait::default();

//     // is too short description
//     mock.expect_add().with(eq("f")).returning(|_| {
//         Err(AppError::InvalidInput(
//             FILE_NAME,
//             "DESCRIPTION is too short",
//         ))
//     });
//     let result = mock.add("f");
//     assert!(result.is_err());
//     let err_task = result.unwrap_err().to_string();
//     assert_eq!(
//         err_task,
//         "Error { code: \"FILE_TEST\", kind: InvalidInput, message: \"DESCRIPTION is too short\" }"
//     );
// }

// #[test]
// fn test_mock_add_should_success() {
//     let mut mock = MockTaskManagerTrait::default();
//     mock.expect_add()
//         .with(eq("test add successfully"))
//         .returning(|_| Ok(task_test::setup_task(2, "test add successfully")));
//     let result = mock.add("test add successfully");
//     assert!(result.is_ok());
//     let task = result.unwrap();
//     assert_eq!(task.id, 2);
//     assert_eq!(task.description, "test add successfully".to_string());
//     assert_eq!(task.status, "todo");
// }

// #[test]
// fn test_mock_update_description_should_fail() {
//     let mut mock = MockTaskManagerTrait::default();
//     mock.expect_update_description()
//         .with(eq(1), eq("f"))
//         .returning(|_, _| {
//             Err(AppError::InvalidInput(
//                 FILE_NAME,
//                 "DESCRIPTION is too short",
//             ))
//         });
//     let result = mock.update_description(1, "f");
//     assert!(result.is_err());
//     let err = result.unwrap_err();
//     assert_eq!(err.kind(), InvalidInput);
//     assert_eq!(
//         err.to_string(),
//         "Error { code: \"FILE_TEST\", kind: InvalidInput, message: \"DESCRIPTION is too short\" }"
//     );
// }

// #[test]
// fn test_mock_update_description_should_success() {
//     let mut mock = MockTaskManagerTrait::default();
//     mock.expect_update_description()
//         .with(eq(1), eq("test update successfully"))
//         .returning(|_, _| {
//             Ok(task_test::setup_task_status(
//                 1,
//                 "test update successfully",
//                 "todo",
//             ))
//         });
//     let result = mock.update_description(1, "test update successfully");
//     assert!(result.is_ok());
//     let task = result.unwrap();
//     assert_eq!(task.id, 1);
//     assert_eq!(task.description, "test update successfully".to_string());
//     assert_eq!(task.status, "todo");
// }

// #[test]
// fn test_mock_delete_should_fail() {
//     let mut mock = MockTaskManagerTrait::default();
//     mock.expect_delete()
//         .with(eq(1))
//         .returning(|_| Err(AppError::InvalidInput(FILE_NAME, "ID not found")));
//     let result = mock.delete(1);
//     assert!(result.is_err());
//     let err = result.unwrap_err();
//     assert_eq!(err.kind(), InvalidInput);
//     assert_eq!(
//         err.to_string(),
//         "Error { code: \"FILE_TEST\", kind: InvalidInput, message: \"ID not found\" }"
//     );
// }

// #[test]
// fn test_mock_delete_should_success() {
//     let mut mock = MockTaskManagerTrait::default();
//     mock.expect_delete().with(eq(1)).returning(|_| Ok(()));
//     let result = mock.delete(1);
//     assert!(result.is_ok());
// }
