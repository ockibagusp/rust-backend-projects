// TDD
// ✅ ❔ ❌
// 3.3. testing: run the commands of `cmd` is done to...
// ------------------------------------------------
// TASK CLI
// 1. to add [MIN=2 chars, MAX=50 chars]
//      error   => the command:
//              => `$ ./task-cli add` or
//              => `$ ./task-cli add foo bar` should be return the usage error message: help for the command of `add`
//      error   => the command `$ ./task-cli add [MIN=1]/[MAX>50]` should be return the usage error message: "DESCRIPTION is too short(min. 2 chars) or too long(max. 50 chars)"
//      success => the command `$ ./task-cli add foo` should be return the success message: "Add task success"
// 2. to update [ID must be a number] [MIN=2 chars, MAX=50 chars]
//      error   => `$ ./task-cli update` or
//              => `$ ./task-cli update i foo bar` should be return the usage error message: "help for the command of `update`"
//      error   => `$ ./task-cli update i` should be return the usage error message: "ID must be a number"
//      error   => `$ ./task-cli update 3 foo` should not be found return the usage error message in ID=3: "not found"
//      success => `$ ./task-cli update 1 foo` should be return the success message: "Update task success"
// 3. to delete [ID must be a number]
//      error   => `$ ./task-cli delete` or
//              => `$ ./task-cli delete 1 extra-arg` should be return the usage error message: help for the command of `delete`
//      error   => `$ ./task-cli delete i` should be return the usage error message: "ID must be a number"
//      error   => `$ ./task-cli delete 3` should not be found return the usage error message in ID=3: "not found"
//      success => `$ ./task-cli delete 1` should be return the success message: "Delete task success"
// ------------------------------------------------
// MARK CLI
// 1. to mark in progress [ID must be a number]
//      error   => `$ ./task-cli mark-in-progress` or
//              => `$ ./task-cli mark-in-progress 1 extra-arg` should be return the usage error message: help for the command of `mark-in-progress`
//      error   => `$ ./task-cli mark-in-progress i` should be return the usage error message: "ID must be a number"
//      error   => `$ ./task-cli mark-in-progress 3` should not be found return the usage error message in ID=3: "not found"
//      success => `$ ./task-cli mark-in-progress 1` should be return the success message: "Mark in progress"
// 2. to mark done [ID must be a number]
//      error   => `$ ./task-cli mark-done` or
//              => `$ ./task-cli mark-done 1 extra-arg` should be return the usage error message: help for the command of `mark-done`
//      error   => `$ ./task-cli mark-done i` should be return the usage error message: "ID must be a number"
//      error   => `$ ./task-cli mark-done 3` should not be found return the usage error message in ID=3: "not found"
//      success => `$ ./task-cli mark-done 1` should be return the success message: "Mark done"
// ------------------------------------------------
// LIST CLI
// 1. to list (all tasks)
//      success => `$ ./task-cli list` should be return the success message: "Lists Task"
// 2. to list todo (tasks)
//      success => `$ ./task-cli list todo` should be return the success message: "Todos Task"
// 3. to list in progress (tasks)
//      success => `$ ./task-cli list in-progress` should be return the success message: "In-Progress Task"
// 4. to list done (tasks)
//      success => `$ ./task-cli list done` should be return the success message: "Done Task"
// ------------------------------------------------
mod tests {
    use crate::cmd::cmd::Cli;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
