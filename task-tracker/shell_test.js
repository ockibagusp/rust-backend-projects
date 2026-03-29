const { exec } = require('child_process');

var test_cases = [
    /**
     * Test cases for add command
     */
    // error: description with 1 character
    {
        command: './task-cli add "f"',
        expected: `Error { code: "TASK_MANAGER", kind: InvalidInput, message: "DESCRIPTION is too short(min. 2 chars) or too long(max. 50 chars)" }\n`
    },
    // error: description with 51 characters
    {
        command: './task-cli add "foo bar baz qux quux corge grault garply waldo fred plugh xyzzy thud"',
        expected: `Error { code: "TASK_MANAGER", kind: InvalidInput, message: "DESCRIPTION is too short(min. 2 chars) or too long(max. 50 chars)" }\n`
    },
    // success: valid description
    {
        command: './task-cli add "This is a valid task description 3"',
        expected: `Add task
------------------
ID: 3
----- Description: This is a valid task description 3`
    },

    /**
     * Test cases for update command
     */
    {
        command: './task-cli update -1 "f"',
        expected: `error: unexpected argument '-1' found`
    },
    {
        command: './task-cli update 1 "f"',
        expected: `Error { code: "TASK_MANAGER", kind: InvalidInput, message: "DESCRIPTION is too short(min. 2 chars) or too long(max. 50 chars)" }\n`
    },
    {
        command: './task-cli update 1 "foo bar baz qux quux corge grault garply waldo fred plugh xyzzy thud"',
        expected: `Error { code: "TASK_MANAGER", kind: InvalidInput, message: "DESCRIPTION is too short(min. 2 chars) or too long(max. 50 chars)" }\n`
    },
    {
        command: './task-cli update 1 "This is an updated task description"',
        expected: `Update task
    ------------------
    ID: 1
    ----- Description: This is an updated task description 1`
    },
    /**
     * Test cases for delete command
     */
    {
        command: './task-cli delete -1',
        expected: `Error: Command failed: ./task-cli delete -1`
    },
    {
        command: './task-cli delete 3',
        expected: `Delete task`
    },

    /**
     * Test cases for mark command
     */
    {
        command: './task-cli mark-in-progress -1',
        expected: `Error: Command failed: ./task-cli mark -1 in-progress`
    },
    {
        command: './task-cli mark-in-progress 1',
        expected: `Mark in progress`
    },
    {
        command: './task-cli mark-done -1',
        expected: `Error: Command failed: ./task-cli mark -1 done`
    },
    {
        command: './task-cli mark-done 2',
        expected: `Mark done`
    },
    /**
     * Test cases for list command
     */
    {
        command: './task-cli list',
        expected: `All Lists
------------------
ID: 1
----- Description: This is a valid task description 1`
    },
    {
        command: './task-cli list todo',
        expected: `Todo Lists
------------------
No lists found.`
    },
    {
        command: './task-cli list in-progress',
        expected: `In Progress Lists
------------------
ID: 1
----- Description: This is a valid task description 1`
    },
    {
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
    var before_test_cases = [
        'This is a valid task description 1',
        'This is a valid task description 2'
    ];
    for (const command of before_test_cases) {
        exec(`./task-cli add "${command}"`, (error, stdout, stderr) => {
            process.stdout.write(`Running: ${command}\n`);
            if (!stdout.includes('Add task')) {
                console.info("\x1b[41m%s\x1b[0m", 'Test passed');
            }
        });
        await sleep(2000); // wait for 2 seconds before running the next command
    }
    console.log('-------------------');

    var total = 0;
    for (const { command, expected } of test_cases) {
        exec(command, (error, stdout, stderr) => {
            process.stdout.write(`Running: ${command}\n`);
            if (error) {
                console.error(`Error: ${error.message}`);
                return;
            }
            if (stderr) {
                console.error(`Stderr: ${stderr}`);
                return;
            }
            if (stdout.includes(expected)) {
                console.info("\x1b[42m%s\x1b[0m", 'Test passed');
                total += 1;
            } else {
                console.log("\x1b[31m%s\x1b[0m", `Stdout: ${stdout}`);
                console.log("\x1b[41m%s\x1b[0m", 'Test failed');
            }
        });
        await sleep(2000); // wait for 2 seconds before running the next command
    }
    process.stdout.write(`\nTotal tests passed: ${total}/${test_cases.length}\n`);
}
runTests();