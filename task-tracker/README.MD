# task-tarcker

Project Task URL: https://roadmap.sh/projects/task-tracker

Rust console app solution for the task-tracker [challenge](https://roadmap.sh/projects/task-tracker) from [roadmap.sh](https://roadmap.sh/).

This is a simple command-line interface (CLI) application for managing tasks.

Task CLI

## Requirements

The application should run from the command line, accept user actions and inputs as arguments, and store the tasks in a JSON file. The user should be able to:

- Add, Update, and Delete tasks

- Mark a task as in progress or done

- List all tasks

- List all tasks that are done

- List all tasks that are not done

- List all tasks that are in progress

Here are some constraints to guide the implementation:

- You can use any programming language to build this project.

- Use positional arguments in command line to accept user inputs.

- Use a JSON file to store the tasks in the current directory.

- The JSON file should be created if it does not exist.

- Use the native file system module of your programming language to interact with the JSON file.

- Do not use any external libraries or frameworks to build this project.

- Ensure to handle errors and edge cases gracefully.

Example
The list of commands and their usage is given below:

---

## Build

`$ make build`

## Shell Test Javascript

`$ make js-test`

### Create a new file for task-cli.json

```
[
    {
        "id": unique,
        "description": "xxx", # =< 10 character
        "status": "todo", # [todo, in-progress, done]
        "created_at": xxx, # date time
        "updated_at": xxx # date time
    }
]
```

---

### Adding a new task

```bash
./task-cli add "Buy groceries"
```

### Output: Task added successfully (ID: 1)

### Updating and deleting tasks

```bash
./task-cli update 1 "Buy groceries and cook dinner"
```

```bash
./task-cli delete 1
```

### Marking a task as in progress or done

```bash
./task-cli mark-in-progress 1
```

```bash
./task-cli mark-done 1
```

### Listing all tasks

```bash
./task-cli list
```

### Listing tasks by status

```bash
./task-cli list done
```

```bash
./task-cli list todo
```

```bash
./task-cli list in-progress
```
