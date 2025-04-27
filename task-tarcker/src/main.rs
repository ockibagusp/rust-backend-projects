mod helps;
mod tasks;
mod tasks_test;

use helps::help_all;
// => add(...); v
use tasks::add;

use std::env::args;

pub fn run(args: &Vec<String>) -> String {
    if args.len() == 1 || args.len() > 3 || (args.len() == 2 && &args[1] == "help") {
        return helps::help_all();
    }

    // 1
    let accept_args = &args[1];

    // add <task>
    if accept_args == "add" {
        if args.len() != 3 {
            return helps::help_add();
        }

        // 2
        let input_args = &args[2];

        let new_task = add(input_args);

        let new_task_str: Vec<String> = vec![
            "Add tast:".to_string(),
            String::from("------------------"),
            format!("ID: {}", new_task.id),
            format!("----- Description: {}", new_task.description),
            format!("----- Status: {}", new_task.status),
            format!("----- Created At: {:?}", new_task.created_at),
            format!("----- Updated At: {:?}", new_task.updated_at),
        ];

        return new_task_str.join("\n");
    }

    // update <id> <task>
    if accept_args == "update" {
        if args.len() != 3 {
            return helps::help_update();
        }

        // 2
        let input_args = &args[2];

        let update_task = tasks::update(1, input_args);

        return String::from("Update task:")
            + "\n"
            + "------------------"
            + "\n"
            + &format!("ID: {}", update_task.id)
            + "\n"
            + &format!("----- Description: {}", update_task.description)
            + "\n"
            + &format!("----- Status: {}", update_task.status)
            + "\n"
            + &format!("----- Created At: {:?}", update_task.created_at)
            + "\n"
            + &format!("----- Updated At: {:?}", update_task.updated_at);
    }

    if accept_args == "delete" {
        if args.len() != 3 {
            return helps::help_delete();
        }

        // 2
        let input_args = &args[2];

        let id: i32 = match input_args.parse() {
            Ok(num) => num,
            Err(_) => {
                return String::from("Error: ID must be a number");
            }
        };

        let delete_task = tasks::delete(id);
        if !delete_task {
            return String::from("Delete task failed");
        }
        return String::from("Delete task success");
    }

    if accept_args == "mark-in-progress" {
        return helps::help_all();
    }

    if accept_args == "mark-done" {
        return helps::help_all();
    }

    // list
    if accept_args == "list" {
        // func => tasks::list(); v
        //         ^^^^^
        //      => add(...); v
        // func => list(); x
        let list = tasks::list();

        let mut list_str: Vec<String> =
            vec!["Lists:".to_string(), String::from("------------------")];
        for task in list {
            list_str.push(format!("ID: {}", task.id));
            list_str.push(format!("----- Description: {}", task.description));
            list_str.push(format!("----- Status: {}", task.status));
            list_str.push(format!("----- Created At: {:?}", task.created_at));
            list_str.push(format!("----- Updated At: {:?}", task.updated_at));
        }
        return list_str.join("\n");
    }

    return help_all();
}

fn main() {
    let args: Vec<String> = args().collect();
    println!("{}", run(&args));
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
    use crate::{
        helps::{self, help_all},
        run,
    };

    static TASK_TRACKER: &str = "task-tracker";

    #[test]
    fn test_not_args() {
        let mut args: Vec<String> = Vec::new();
        args.push(String::from(TASK_TRACKER));
        assert_eq!(args.len(), 1);
        assert_eq!(run(&args), help_all());

        args.push("first_err".to_string());
        assert_eq!(args.len(), 2);
        assert_eq!(run(&args), help_all());

        args.push(String::from("second_err"));
        args.push(String::from("third_err"));
        assert_eq!(args.len(), 4);
        assert_eq!(run(&args), help_all());
    }

    #[test]
    fn test_add_empty() {
        let args: Vec<String> = vec![String::from(TASK_TRACKER), String::from("add")];
        assert_eq!(args.len(), 2);
        assert_eq!(args[1], "add");
        assert_eq!(run(&args), helps::help_add());
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
        let expected_output = String::from(
            "Add tast:\n------------------\nID: 1\n----- Description: test buy milk\n----- Status: todo\n----- Created At: 2025-04-10T10:10:10+07:00\n----- Updated At: 2025-04-10T10:10:10+07:00",
        );
        assert_eq!(run(&args), expected_output);
    }

    #[test]
    fn test_update_empty() {
        let args: Vec<String> = vec![String::from(TASK_TRACKER), String::from("update")];
        assert_eq!(args.len(), 2);
        assert_eq!(args[1], "update");
        assert_eq!(run(&args), helps::help_update());
    }

    #[test]
    fn test_update_success() {
        let args: Vec<String> = vec![
            String::from(TASK_TRACKER),
            String::from("update"),
            String::from("test buy 3 eggs"),
        ];

        assert_eq!(args.len(), 3);
        assert_eq!(args[1], "update");
        let expected_output = String::from(
            "Update task:\n------------------\nID: 1\n----- Description: test buy 3 eggs\n----- Status: todo\n----- Created At: 2025-04-10T10:10:10+07:00\n----- Updated At: 2025-04-10T14:10:10+07:00",
        );
        assert_eq!(run(&args), expected_output);
    }

    #[test]
    fn test_not_list() {
        let args: Vec<String> = vec![String::from(TASK_TRACKER), String::from("fail")];
        assert_eq!(args.len(), 2);
        assert_eq!(run(&args), help_all());
    }

    #[test]
    fn test_args_help_success() {
        let args: Vec<String> = vec![String::from(TASK_TRACKER), String::from("help")];
        assert_eq!(args.len(), 2);
        assert_eq!(run(&args), help_all());
    }

    #[test]
    fn test_args_list_success() {
        let args: Vec<String> = vec![String::from(TASK_TRACKER), String::from("list")];

        assert_eq!(args.len(), 2);
        assert_eq!(args[1], "list");
        let expected_output = String::from(
            "Lists:\n------------------\nID: 1\n----- Description: buy milk\n----- Status: todo\n----- Created At: 2025-04-10T10:10:10+07:00\n----- Updated At: 2025-04-10T10:10:10+07:00\nID: 2\n----- Description: buy bread\n----- Status: in-progress\n----- Created At: 2025-04-12T12:10:10+07:00\n----- Updated At: 2025-04-12T12:10:10+07:00",
        );
        assert_eq!(run(&args), expected_output);
    }
}
