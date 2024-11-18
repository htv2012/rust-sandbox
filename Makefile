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

### Format, edit, and run main.rs
dev:
	$(MAKE) -s -f $(MAKEFILE_LIST) format
	$(MAKE) -s -f $(MAKEFILE_LIST) edit
	$(MAKE) -s -f $(MAKEFILE_LIST) run

### Edit main.rs
edit:
	$(EDITOR) src/main.rs

### Edit lib.rs
editl:
	$(EDITOR) src/lib.rs

### Format the sources
format:
	cargo fmt --all

### List the projects
list:
	tree -L 3 -d -I src

### Run main.rs
run:
	cargo run -q --

### Perform unit tests
test:
	cargo test

.PHONY: clean dev edit editl format help list run test