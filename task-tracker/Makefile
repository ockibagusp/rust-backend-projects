link := task-tracker
link_after := task-cli

remove_json := @rm -f tasks_test.json

run:
action:
	cargo run

test:
	cargo test

build:
	cargo build

# create a symbolic link
	ln -sf target/debug/$(link) $(link_after)

js-test:
# if tasks_test.json exists, remove it
	$(remove_json)

	@make build
	@ENV_JSON=tasks_test.json node shell_test.js

# remove it
	$(remove_json)
	

