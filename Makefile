#
# The master Makefile for all Rust projects
#

### Show this message
help:
	@make-list-targets $(MAKEFILE_LIST)

### Delete generated files and directories
clean:
	find -type d -exec test -d '{}/target' \; -print | \
	while IFS= read -r dir; do \
		(cd "$$dir" && echo "Clean: $$dir" ; cargo clean); \
	done

### Development cycle
dev: format edit run

### Edit main.rs
edit:
	$(EDITOR) src/main.rs

### Edit lib.rs
editl:
	$(EDITOR) src/lib.rs

### Format the sources
format:
	cargo fmt --all

### List project's files, excluding the target dir
list:
	tree -I target

### Run main.rs
run:
	cargo run -q --

### Perform unit tests
test:
	cargo test

### List the projects
tree:
	tree -L 3 -d -I src $(HOME)/Projects/rust-sandbox | less -R

.PHONY: clean dev edit editl format help run test tree