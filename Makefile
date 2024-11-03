list:
	tree -L 3 -d -I src

clean:
	find -type d -exec test -d '{}/target' \; -print | \
	while IFS= read -r dir; do \
		(cd "$$dir" && echo "Clean: $$dir" ; cargo clean); \
	done
