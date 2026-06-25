use crate::application::mark_use_cases::MarkUseCaseTrait;
use crate::domain::task::Task;
use crate::error::AppError;

pub struct CmdMarkHandler {
    // Presentation depends directly on the Application use case
    pub use_case: Box<dyn MarkUseCaseTrait>,
}

impl CmdMarkHandler {
    pub fn new(use_case: Box<dyn MarkUseCaseTrait>) -> Self {
        Self { use_case }
    }

    pub fn handle_mark_in_progress(&mut self, id: i32) -> Result<Task, AppError> {
        return self.use_case.mark_in_progress(id);
    }

    pub fn handle_mark_done(&mut self, id: i32) -> Result<Task, AppError> {
        return self.use_case.mark_done(id);
    }
}
