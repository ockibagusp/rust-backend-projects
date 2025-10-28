# rust-backend-projects

Project ideas to take you from beginner to advanced in Backend.

## 1. Task CLI

id: unique
description: =< 10 karaker
status: todo, in-progress, done
created_at: unix time
updated_at: unix time

bash

# Adding a new task

task-cli add "Buy groceries"

# Output: Task added successfully (ID: 1)

# Updating and deleting tasks

task-cli update 1 "Buy groceries and cook dinner"
task-cli delete 1

# Marking a task as in progress or done

task-cli mark-in-progress 1
task-cli mark-done 1

# Listing all tasks

task-cli list

# Listing tasks by status

task-cli list done
task-cli list todo
task-cli list in-progress

### task-cli.json

`{
  "next_id": 3,
  "tasks": [
    {
      "id": 1,
      "description": "Buy groceries",
      "status": "done",
      "created_at": "2025-10-13 01:07:06.072493 +07:00",
      "updated_at": "2025-10-13 01:07:06.072493 +07:00"
    },
    {
      "id": 2,
      "description": "Buy cook dinner",
      "status": "in-progress",
      "created_at": "2025-10-13 14:07:06.072493 +07:00",
      "updated_at": "2025-10-13 19:07:06.072493 +07:00"
    }
  ]
}`
