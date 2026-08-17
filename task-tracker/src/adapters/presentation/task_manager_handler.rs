use crate::application::use_cases::task_manager::TaskManagerUseCase;
use crate::domain::error::AppError;
use crate::domain::task::Task;

pub struct CmdTaskManagerHandler {
    // Presentation depends directly on the Application use case
    pub use_case: Box<TaskManagerUseCase>,
}

impl CmdTaskManagerHandler {
    pub fn new(use_case: Box<TaskManagerUseCase>) -> Self {
        Self { use_case }
    }

    pub fn handle_add_tasks(&self, input: &str) -> Result<Task, AppError> {
        return self.use_case.add(input);
    }

    pub fn handle_update_description(&self, id: i32, description: &str) -> Result<Task, AppError> {
        return self.use_case.update_description(id, description);
    }

    pub fn handle_updates(
        &self,
        id: i32,
        update_task: &mut Task,
        desc_status: i32,
    ) -> Result<Task, AppError> {
        return self.use_case.updates(id, update_task, desc_status);
    }

    pub fn handle_delete(&self, id: i32) -> Result<(), AppError> {
        return self.use_case.delete(id);
    }
}
