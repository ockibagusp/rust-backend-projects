use crate::domain::task::Task;
use crate::error::AppError;
use core::result::Result;

pub trait MarkRepository {
    fn mark_in_progress(&mut self, id: i32) -> Result<Task, AppError>;
    fn mark_done(&mut self, id: i32) -> Result<Task, AppError>;
}
