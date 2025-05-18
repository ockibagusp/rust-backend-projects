pub fn help_all() -> String {
    return format!(
        "Usage: 
        $ task-cli <accept> <input>
        
        Example:
        # Adding a new task, updating and deleting
        $ task-cli add \"buy milk\"
        $ task-cli update 1 \"buy milk and 2 eggs\"
        $ task-cli delete 1

        # Marking a task as progress or done
        $ task-cli mark-in-progress 1
        $ task-cli mark-done 1
        
        # Listing all tasks, done tasks and todo tasks
        $ task-cli list
        $ task-cli list done
        $ task-cli list todo
        $ task-cli list in-progress
    "
    );
}

pub fn help_add() -> String {
    return format!(
        "Usage: 
        $ task-cli add <task>
        
        Example:
        $ task-cli add \"buy milk\"
    "
    );
}

pub fn help_update() -> String {
    return format!(
        "Usage: 
        $ task-cli update <id> <task>
        
        Example:
        $ task-cli update 1 \"buy milk and 2 eggs\"
    "
    );
}

pub fn help_delete() -> String {
    return format!(
        "Usage: 
        $ task-cli delete <id>
        
        Example:
        $ task-cli delete 1
    "
    );
}
