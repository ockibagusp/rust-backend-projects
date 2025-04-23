link := task-tarcker
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
	

