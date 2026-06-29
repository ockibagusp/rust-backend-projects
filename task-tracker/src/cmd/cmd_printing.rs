use crate::domain::task::Task;
use chrono::{DateTime, FixedOffset};

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

pub fn open_task_list_for_title_str(title: &str, list: Vec<Task>) -> String {
    let tasks_str: String;
    if list.is_empty() {
        tasks_str = String::from("No lists found.");
    } else {
        let mut list_str: Vec<String> = vec![];
        for task in list {
            list_str.push(open_task_str(&task));
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

fn get_datetimes_to_string(date_time: &DateTime<FixedOffset>) -> String {
    return date_time.format("%d-%m-%Y %H:%M:%S").to_string();
}
