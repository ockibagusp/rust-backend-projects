use crate::cmd::{
    domain::{
        models::{Task, TaskStatus},
        models_test::{setup_task, setup_task_status},
    },
    infrastructure::storages::storage::StorageTrait,
};
use crate::config;

#[derive(Clone)]
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
