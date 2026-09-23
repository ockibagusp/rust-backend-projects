use crate::cmd::{
    application::use_cases::mark::{MarkDoneUseCase, MarkDto, MarkInProgressUseCase},
    domain::{error::AppError, models::Task},
};

pub struct CmdMarkInProgressHandler {
    // Presentation depends directly on the Application use case
    pub use_case: Box<MarkInProgressUseCase>,
}

impl CmdMarkInProgressHandler {
    pub fn new(use_case: Box<MarkInProgressUseCase>) -> Self {
        Self { use_case }
    }

    pub fn execute(&self, request: MarkDto) -> Result<Task, AppError> {
        return self.use_case.execute(request);
    }
}

pub struct CmdMarkDoneHandler {
    // Presentation depends directly on the Application use case
    pub use_case: Box<MarkDoneUseCase>,
}

impl CmdMarkDoneHandler {
    pub fn new(use_case: Box<MarkDoneUseCase>) -> Self {
        Self { use_case }
    }

    pub fn execute(&self, request: MarkDto) -> Result<Task, AppError> {
        return self.use_case.execute(request);
    }
}
