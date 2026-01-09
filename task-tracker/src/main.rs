use crate::task::task_manager::{TaskManager, TaskManagerTrait};

use core::result::Result;

mod file;
mod help;
mod list;
mod task;

use help::help_all;
// => add(...); v

pub fn run(args: &Vec<String>, task_manager: &mut TaskManager) -> String {
    if args.len() == 1 || args.len() > 3 || (args.len() == 2 && &args[1] == "help") {
        return help::help_all();
    }

    // 1
    let accept_args = &args[1];

    // add <task>
    if accept_args == "add" {
        if args.len() != 3 {
            return help::help_add();
        }
        // 2
        let input_args = &args[2];

        let task_result = task_manager.add(input_args);
        if let Err(e) = task_result {
            return e.to_string();
        }
        let task = &task_result.unwrap();
        let task_manager_str: Vec<String> = vec![
            "Add tast:".to_string(),
            String::from("------------------"),
            format!("ID: {}", task.id),
            format!("----- Description: {}", task.description),
            format!("----- Status: {}", task.status),
            format!("----- Created At: {:?}", task.created_at),
            format!("----- Updated At: {:?}", task.updated_at),
        ];

        return task_manager_str.join("\n");
    }

    // update <id> <task>
    if accept_args == "update" {
        if args.len() != 4 {
            return help::help_update();
        }
        // 2
        let input_args = &args[2];

        let mut update_task = <TaskManager as TaskManagerTrait>::new("task-cli.json");
        let task_result = update_task.update_description(1, input_args);

        let Ok(task) = task_result else {
            return String::from("Error updating task: ")
                + task_result.unwrap_err().to_string().as_str();
        };
        return String::from("Update task:")
            + "\n"
            + "------------------"
            + "\n"
            + &format!("ID: {}", task.id)
            + "\n"
            + &format!("----- Description: {}", task.description)
            + "\n"
            + &format!("----- Status: {}", task.status)
            + "\n"
            + &format!("----- Created At: {:?}", task.created_at)
            + "\n"
            + &format!("----- Updated At: {:?}", task.updated_at);
    }

    // if accept_args == "delete" {
    //     if args.len() != 3 {
    //         return helps::help_delete();
    //     }

    //     // 2
    //     let input_args = &args[2];

    //     let id: i32 = match input_args.parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             return String::from("Error: ID must be a number");
    //         }
    //     };

    //     let delete_task = task::delete(id);
    //     if !delete_task {
    //         return String::from("Delete task failed");
    //     }
    //     return String::from("Delete task success");
    // }

    // if accept_args == "mark-in-progress" {
    //     return helps::help_all();
    // }

    // if accept_args == "mark-done" {
    //     return helps::help_all();
    // }

    // // list
    // if accept_args == "list" {
    //     // func => tasks::list(); v
    //     //         ^^^^^
    //     //      => add(...); v
    //     // func => list(); x
    //     let list = task::list();

    //     let mut list_str: Vec<String> =
    //         vec!["Lists:".to_string(), String::from("------------------")];
    //     for task in list {
    //         list_str.push(format!("ID: {}", task.id));
    //         list_str.push(format!("----- Description: {}", task.description));
    //         list_str.push(format!("----- Status: {}", task.status));
    //         list_str.push(format!("----- Created At: {:?}", task.created_at));
    //         list_str.push(format!("----- Updated At: {:?}", task.updated_at));
    //     }
    //     return list_str.join("\n");
    // }

    return help_all();
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    let mut new_task = <TaskManager as TaskManagerTrait>::new("tasks.json");

    println!("{}", run(&args, &mut new_task));

    Ok(())
}

// TDD
// ✅ ❔ ❌
// 1. tambah, perubahan, hapus untuk tugas
// ---------------------------
// 1.1. tambah tugas ❔
// 1.2. perubahan tugas ❔
// 1.3. hapus tugas ❔
#[cfg(test)]
mod tests {
    use crate::TaskManager;
    use crate::TaskManagerTrait;
    use crate::{
        help::{self, help_all},
        run,
    };

    static TASK_TRACKER: &str = "task-tracker";

    #[test]
    fn test_not_args() {
        let mut args: Vec<String> = Vec::new();
        let mut new_task = <TaskManager as TaskManagerTrait>::new("test-tasks.json");
        args.push(String::from(TASK_TRACKER));
        assert_eq!(args.len(), 1);
        assert_eq!(run(&args, &mut new_task), help_all());

        args.push("first_err".to_string());
        assert_eq!(args.len(), 2);
        assert_eq!(run(&args, &mut new_task), help_all());

        args.push(String::from("second_err"));
        args.push(String::from("third_err"));
        assert_eq!(args.len(), 4);
        assert_eq!(run(&args, &mut new_task), help_all());
    }

    #[test]
    fn test_add_empty() {
        let args: Vec<String> = vec![String::from(TASK_TRACKER), String::from("add")];
        let mut new_task = <TaskManager as TaskManagerTrait>::new("test-tasks.json");

        assert_eq!(args.len(), 2);
        assert_eq!(args[1], "add");
        assert_eq!(run(&args, &mut new_task), help::help_add());
    }

    #[test]
    fn test_args_add_success() {
        let args: Vec<String> = vec![
            String::from(TASK_TRACKER),
            String::from("add"),
            String::from("test buy milk"),
        ];

        assert_eq!(args.len(), 3);
        assert_eq!(args[1], "add");
        assert_eq!(args[2], "test buy milk");
    }

    #[test]
    fn test_update_empty() {
        let args: Vec<String> = vec![String::from(TASK_TRACKER), String::from("update")];
        let mut new_task = <TaskManager as TaskManagerTrait>::new("test-tasks.json");

        assert_eq!(args.len(), 2);
        assert_eq!(args[1], "update");
        assert_eq!(run(&args, &mut new_task), help::help_update());
    }

    #[test]
    fn test_update_success() {
        let args: Vec<String> = vec![
            String::from(TASK_TRACKER),
            String::from("update"),
            String::from("test buy 3 eggs"),
        ];
        let mut new_task = <TaskManager as TaskManagerTrait>::new("test-tasks.json");

        assert_eq!(args.len(), 3);
        assert_eq!(args[1], "update");
    }

    #[test]
    fn test_not_list() {
        let args: Vec<String> = vec![String::from(TASK_TRACKER), String::from("fail")];
        let mut new_task = <TaskManager as TaskManagerTrait>::new("test-tasks.json");

        assert_eq!(args.len(), 2);
        assert_eq!(run(&args, &mut new_task), help_all());
    }

    #[test]
    fn test_args_help_success() {
        let args: Vec<String> = vec![String::from(TASK_TRACKER), String::from("help")];
        let mut new_task = <TaskManager as TaskManagerTrait>::new("test-tasks.json");

        assert_eq!(args.len(), 2);
        assert_eq!(run(&args, &mut new_task), help_all());
    }

    #[test]
    fn test_args_list_success() {
        let args: Vec<String> = vec![String::from(TASK_TRACKER), String::from("list")];

        assert_eq!(args.len(), 2);
        assert_eq!(args[1], "list");
    }
}
