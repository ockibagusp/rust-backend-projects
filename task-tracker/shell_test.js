const { exec } = require('child_process');

var test_cases = [
    /**
     * Test cases for add command
     */
    // error: description with 1 character
    {
        name: 'add command with invalid description (1 character) should fail',
        command: './task-cli add "f"',
        expected: `code   : TASK_MANAGER
kind   : InvalidInput
message: "DESCRIPTION is too short(min. 2 chars) or too long(max. 50 chars)"`
    },
    // error: description with 51 characters
    {
        name: 'add command with invalid description (51 characters) should fail',
        command: './task-cli add "foo bar baz qux quux corge grault garply waldo fred plugh xyzzy thud"',
        expected: `code   : TASK_MANAGER
kind   : InvalidInput
message: "DESCRIPTION is too short(min. 2 chars) or too long(max. 50 chars)"`
    },
    // success: valid description
    {
        name: 'add command with valid description should succeed',
        command: './task-cli add "This is a valid task description 3"',
        expected: `Add task
------------------
ID: 3
----- Description: This is a valid task description 3\n`
    },

    /**
     * Test cases for update command
     */
    {
        name: 'update command with missing arguments should fail',
        command: './task-cli update -1 "f"',
        expected: 'Usage: task-cli update <ID> <DESCRIPTION>'
    },
    {
        name: 'update command with invalid description (1 character) should fail',
        command: './task-cli update 1 "f"',
        expected: `code   : TASK_MANAGER
kind   : InvalidInput
message: "DESCRIPTION is too short(min. 2 chars) or too long(max. 50 chars)"`
    },
    {
        name: 'update command with invalid description (51 characters) should fail',
        command: './task-cli update 1 "foo bar baz qux quux corge grault garply waldo fred plugh xyzzy thud"',
        expected: `code   : TASK_MANAGER
kind   : InvalidInput
message: "DESCRIPTION is too short(min. 2 chars) or too long(max. 50 chars)"`
    },
    {
        name: 'update command with valid description should succeed',
        command: './task-cli update 1 "This is an updated task description 1"',
        expected: `Update task
------------------
ID: 1
----- Description: This is an updated task description 1`
    },
    // update with the same description and status should fail
    {
        name: 'update command with identical description and status should fail',
        command: './task-cli update 1 "This is an updated task description 1"',
        expected: `code   : TASK_MANAGER
kind   : InvalidInput
message: "DESCRIPTION or STATUS is not identical"`
    },
    /**
     * Test cases for delete command
     */
    {
        name: 'delete command with missing argument should fail',
        command: './task-cli delete',
        expected: `Usage: task-cli delete <ID>`
    },
    {
        name: 'delete command with valid ID should succeed',
        command: './task-cli delete 3',
        expected: `Delete task`
    },

    /**
     * Test cases for mark command
     */
    {
        name: 'mark-in-progress command with missing argument should fail',
        command: './task-cli mark-in-progress',
        expected: `Usage: task-cli mark-in-progress <ID>`
    },
    // mark in progress with ID: 99, it should have failed
    {
        name: 'mark-in-progress command with invalid ID should fail',
        command: './task-cli mark-in-progress 99',
        expected: `code   : MARK
kind   : NotFound
message: "ID is not found"`
    },
    // mark in progress with ID: 1, it should have failed
    {
        name: 'mark-in-progress command with valid ID should succeed',
        command: './task-cli mark-in-progress 1',
        expected: `Mark in progress`
    },
    {
        name: 'mark-done command with missing argument should fail',
        command: './task-cli mark-done',
        expected: `Usage: task-cli mark-done <ID>`
    },
    // mark done with ID: 99, it should have failed
    {
        name: 'mark-done command with invalid ID should fail',
        command: './task-cli mark-done 99',
        expected: `code   : MARK
kind   : NotFound
message: "ID is not found"`
    },
    // mark done with ID: 1, it should have success
    {
        name: 'mark-done command with valid ID should succeed',
        command: './task-cli mark-done 2',
        expected: `Mark done`
    },
    /**
     * Test cases for list command
     */
    {
        name: 'list command with no arguments should succeed',
        command: './task-cli list',
        expected: `All Lists
------------------
ID: 1
----- Description: This is an updated task description 1`
    },
    {
        name: 'list command with invalid status should fail',
        command: './task-cli list todo',
        expected: `Todo Lists
------------------
No lists found.`
    },
    {
        name: 'list command with in-progress status should succeed',
        command: './task-cli list in-progress',
        expected: `In Progress Lists
------------------
ID: 1
----- Description: This is an updated task description 1`
    },
    {
        name: 'list command with done status should succeed',
        command: './task-cli list done',
        expected: `Done Lists
------------------
ID: 2
----- Description: This is a valid task description 2`
    }
];

const sleep = (ms) => new Promise(resolve => setTimeout(resolve, ms));
async function runTests() {
    // repeat two more times
    for (var i = 0; i < 2; i++) {
        var command = `./task-cli add "This is a valid task description ${i + 1}"`;
        exec(command, (error, stdout, stderr) => {
            process.stdout.write(`Running: ${command}\n`);
        });
        await sleep(1500); // wait for 1.5 seconds before running the next command
    }
    console.log('-------------------');

    var total = 0;
    for (const { name, command, expected } of test_cases) {
        exec(command, (error, stdout, stderr) => {
            process.stdout.write(`Name: ${name}\n`);
            process.stdout.write(`Running: ${command}\n`);
            var output = error || stderr || stdout;
            if (output) {
                if ((output.message && output.message.includes(expected)) || output.includes(expected)) {
                    console.info("\x1b[42m%s\x1b[0m", 'Test passed');
                    total += 1;
                } else {
                    console.log("\x1b[32m%s\x1b[0m", `expected  : ${expected}`);
                    console.log("\x1b[31m%s\x1b[0m", `output    : ${output}`);
                    console.log("\x1b[41m%s\x1b[0m", 'Test failed');
                }
                return;
            }
        });
        await sleep(1500); // wait for 1.5 seconds before running the next command
    }
    var logo_img = "❌";
    if (total === test_cases.length) {
        logo_img = "✅";
    }
    process.stdout.write(`\nTotal tests passed: ${total}/${test_cases.length} ${logo_img}\n`);
}
runTests();