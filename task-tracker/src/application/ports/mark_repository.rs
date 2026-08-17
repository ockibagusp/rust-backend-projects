use crate::domain::error::AppError;
use crate::domain::task::Task;
use core::result::Result;

// TDD
// ✅ ❔ ❌
// 2.3. buatlah trait MarkRepository dengan method mark_in_progress, mark_done, find_by_list ✅
// => 2.3. create the MarkRepository trait with method mark_in_progress, mark_done, find_by_list
pub trait MarkRepository {
    fn mark_in_progress(&self, id: i32, update_task: &mut Task) -> Result<Task, AppError>;
    fn mark_done(&self, id: i32, update_task: &mut Task) -> Result<Task, AppError>;

    fn find_by_list(&self) -> Vec<Task>;
}
