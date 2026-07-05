use crate::domain::task::Task;
use crate::error::AppError;
use core::result::Result;

pub trait TaskManagerRepository {
    // ? fn find_by_id_mut(&mut self, id: i32, update_task: &Task) -> ();
    // some operations with CRUD
    fn add(&mut self, input: &str) -> Result<Task, AppError>;
    fn update_description(&mut self, id: i32, description: &str) -> Result<Task, AppError>;
    fn updates(
        &mut self,
        id: i32,
        update_task: &mut Task,
        desc_status: i32,
    ) -> Result<Task, AppError>;
    fn delete(&mut self, id: i32) -> Result<(), AppError>;

    fn find_by_list(&self) -> Vec<Task>;
}
