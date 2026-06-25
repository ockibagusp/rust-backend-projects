use crate::application::task_manager_use_cases::TaskManagerUseCaseTrait;
use crate::domain::task::Task;
use crate::error::AppError;

pub struct CmdTaskManagerHandler {
    // Presentation depends directly on the Application use case
    pub use_case: Box<dyn TaskManagerUseCaseTrait>,
}

impl CmdTaskManagerHandler {
    pub fn new(use_case: Box<dyn TaskManagerUseCaseTrait>) -> Self {
        Self { use_case }
    }

    pub fn handle_add_tasks(&mut self, input: &str) -> Result<Task, AppError> {
        return self.use_case.add(input);
    }

    pub fn handle_update_description(
        &mut self,
        id: i32,
        description: &str,
    ) -> Result<Task, AppError> {
        return self.use_case.update_description(id, description);
    }

    pub fn handle_updates(
        &mut self,
        id: i32,
        update_task: &mut Task,
        desc_status: i32,
    ) -> Result<Task, AppError> {
        return self.use_case.updates(id, update_task, desc_status);
    }

    pub fn handle_delete(&mut self, id: i32) -> Result<(), AppError> {
        return self.use_case.delete(id);
    }
}
