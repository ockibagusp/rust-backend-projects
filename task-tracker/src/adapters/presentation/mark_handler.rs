use crate::application::use_cases::mark::MarkUseCase;
use crate::domain::error::AppError;
use crate::domain::task::Task;

pub struct CmdMarkHandler {
    // Presentation depends directly on the Application use case
    pub use_case: Box<MarkUseCase>,
}

impl CmdMarkHandler {
    pub fn new(use_case: Box<MarkUseCase>) -> Self {
        Self { use_case }
    }

    pub fn handle_mark_in_progress(&self, id: i32) -> Result<Task, AppError> {
        return self.use_case.mark_in_progress(id);
    }

    pub fn handle_mark_done(&self, id: i32) -> Result<Task, AppError> {
        return self.use_case.mark_done(id);
    }
}
