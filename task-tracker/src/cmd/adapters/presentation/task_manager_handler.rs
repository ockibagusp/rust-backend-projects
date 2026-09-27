use crate::cmd::{
    application::use_cases::task_manager::*,
    domain::{error::AppError, models::Task},
};

pub struct CmdCreateTaskManagerHandler<R>
where
    R: TaskManagerRepository,
{
    pub use_case: CreateTaskManagerUseCase<R>,
}

impl<R> CmdCreateTaskManagerHandler<R>
where
    R: TaskManagerRepository,
{
    pub fn new(use_case: CreateTaskManagerUseCase<R>) -> Self {
        Self { use_case }
    }

    pub fn execute(&mut self, request: CreateTaskManagerDto) -> Result<Task, AppError> {
        self.use_case.execute(request)
    }
}

pub struct CmdUpdateTaskManagerHandler<R>
where
    R: TaskManagerRepository,
{
    pub use_case: UpdateTaskManagerUseCase<R>,
}

impl<R> CmdUpdateTaskManagerHandler<R>
where
    R: TaskManagerRepository,
{
    pub fn new(use_case: UpdateTaskManagerUseCase<R>) -> Self {
        Self { use_case }
    }

    pub fn execute_update_desc(&mut self, request: UpdateTaskManagerDto) -> Result<Task, AppError> {
        self.use_case.execute_update_desc(request)
    }

    pub fn execute_update(
        &mut self,
        id: i32,
        update_task: &mut Task,
        desc_status: i32,
    ) -> Result<Task, AppError> {
        self.use_case.execute_update(id, update_task, desc_status)
    }
}

pub struct CmdDeleteTaskManagerHandler<R>
where
    R: TaskManagerRepository,
{
    pub use_case: DeleteTaskManagerUseCase<R>,
}

impl<R> CmdDeleteTaskManagerHandler<R>
where
    R: TaskManagerRepository,
{
    pub fn new(use_case: DeleteTaskManagerUseCase<R>) -> Self {
        Self { use_case }
    }

    pub fn execute(&mut self, request: DeleteTaskManagerDto) -> Result<(), AppError> {
        self.use_case.execute(request)
    }
}
