#!/bin/sh
find -type d -exec test -d '{}/target' \; -print | \
while IFS= read -r dir
do
    (cd "$dir" && echo "Clean: $dir" ; cargo clean)
done
