use crate::domain::task::Task;
use chrono::{DateTime, FixedOffset};

pub struct OpenTasks {
    pub tasks_to_specify: Vec<Task>,
}

pub trait OpenTasksTrait {
    fn new(tasks: Vec<Task>) -> Self
    where
        Self: Sized;

    fn list(&self) -> String;
    fn todo(&self) -> String;
    fn in_progress(&self) -> String;
    fn done(&self) -> String;

    fn list_for_title_str(&self, title: &str) -> String;
}

impl OpenTasksTrait for OpenTasks {
    fn new(tasks: Vec<Task>) -> Self
    where
        Self: Sized,
    {
        return OpenTasks {
            tasks_to_specify: tasks,
        };
    }

    fn list(&self) -> String {
        self.list_for_title_str("All Lists")
    }

    fn todo(&self) -> String {
        self.list_for_title_str("Todo Lists")
    }

    fn in_progress(&self) -> String {
        self.list_for_title_str("In Progress Lists")
    }

    fn done(&self) -> String {
        self.list_for_title_str("Done Lists")
    }

    fn list_for_title_str(&self, title: &str) -> String {
        let tasks_str: String;
        if self.tasks_to_specify.is_empty() {
            tasks_str = String::from("No lists found.");
        } else {
            let mut list_str: Vec<String> = vec![];
            for task in &self.tasks_to_specify {
                list_str.push(open_task_str(task));
            }
            tasks_str = list_str.join("\n");
        }

        return format!(
            "{}\n{}\n{}\n{}",
            String::from(title),
            String::from("------------------"),
            tasks_str,
            String::from("++++++++++++++++++")
        );
    }
}

pub fn open_task_str(task: &Task) -> String {
    return format!(
        "ID: {}\n----- Description: {}\n----- Status     : {}\n----- Created At : {}\n----- Updated At : {}",
        task.id,
        task.description,
        task.status,
        get_datetimes_to_string(&task.created_at),
        get_datetimes_to_string(&task.updated_at)
    );
}

pub fn open_task_title_str(title: &str, task: Task) -> String {
    return format!(
        "{}\n{}\n{}",
        String::from(title),
        String::from("------------------"),
        open_task_str(&task)
    );
}

fn get_datetimes_to_string(date_time: &DateTime<FixedOffset>) -> String {
    return date_time.format("%d-%m-%Y %H:%M:%S").to_string();
}
