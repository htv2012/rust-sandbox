This course is from Coursera

# Key Terms

* **Binary application/package**: Executables generated from Rust source
  files, typically containing a main function.
* **Library**: A collection of Rust modules providing functionality meant to
  be shared among multiple projects.
* **Cargo.toml**: A configuration file read by Cargo, listing metadata
  (e.g., name, version) and dependencies required by the package.
* **Shadowing**: Reassigning a variable to a new value while preserving its
  original binding, enabling changes to its type or scope.
* **Control Flow**: Conditional execution paths based on evaluation of
  logical expressions, including if, else, and else if clauses.
* **Scope**: A region within source code where names (e.g., functions,
  variables) are accessible; determined by enclosing braces ({}) or
  indentation levels.
* **Semicolons**: Terminators denoting statement boundaries, required in
  most cases except inside blocks, expressions, and macros.

# Loop and Control Flow Key Terms

* **loop**: A keyword used for an infinite loop, which can be exited using a
  break statement.
* **while**: A conditional loop that continues as long as its condition is
  true.
* **for**: A loop that iterates through elements of a collection or range.
* **break**: A control flow keyword to exit the current innermost loop
  early.
* **mutability**: The ability for a variable to have its value changed
  during runtime by marking it as mutable with the mut keyword
* **option**: A Rust enum type that can be either Some(value) or None, used
  to represent optional values.
* **continue**: A control flow keyword to skip an iteration and move on to
  the next one in the same loop.
* **if let**: A pattern matching construct that allows you to conditionally
  bind variables based on their match against a given value or pattern.
* **sum**: An enum type wrapper around Option<T> which can be either
  Some(value) or None.
* **range**: Represents a sequence of numbers, often used in loops for
  iteration purposes.
* **shadowing**: A variable redeclaration with the same name but different
  value and/or scope within the same context.

# How to Create a New Project

```bash
# Create a new project
cargo new my_project

# Run it
cd my_project
cargo run

# Edit
vim src/main.rs

# Build
cargo build

# Clean
cargo clean
```

