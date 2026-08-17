use crate::domain::{error::AppError, task::Task};
use core::result::Result;

// TDD
// ✅ ❔ ❌
// 2.2. buatlah trait TaskManagerRepository dengan method add, update_description, updates, delete, find_by_list ✅
// => 2.2. create the TaskManagerRepository trait with method add, update_description, updates, delete, find_by_list
pub trait TaskManagerRepository {
    // ? fn find_by_id_mut(&mut self, id: i32, update_task: &Task) -> ();
    // some operations with CRUD
    fn add(&self, add_task: Task) -> Result<Task, AppError>;
    fn update_description(&self, id: i32, update_task: &Task) -> Result<Task, AppError>;
    fn updates(&self, id: i32, update_task: &Task) -> Result<Task, AppError>;
    fn delete(&self, id: i32) -> Result<(), AppError>;

    fn find_by_list(&self) -> Vec<Task>;
}
