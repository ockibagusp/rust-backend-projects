link := task-tracker
link_after := task-cli

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
	@make build
	@ENV_JSON=tasks_test.json node shell_test.js
	
	@rm -f tasks_test.json
	

