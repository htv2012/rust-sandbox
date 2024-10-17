This course is from Coursera

# Key Terms

* Binary application/package: Executables generated from Rust source
  files, typically containing a main function.
* Library: A collection of Rust modules providing functionality meant to
  be shared among multiple projects.
* Cargo.toml: A configuration file read by Cargo, listing metadata
  (e.g., name, version) and dependencies required by the package.
* Shadowing: Reassigning a variable to a new value while preserving its
  original binding, enabling changes to its type or scope.
* Control Flow: Conditional execution paths based on evaluation of
  logical expressions, including if, else, and else if clauses.
* Scope: A region within source code where names (e.g., functions,
  variables) are accessible; determined by enclosing braces ({}) or
  indentation levels.
* Semicolons: Terminators denoting statement boundaries, required in
  most cases except inside blocks, expressions, and macros.

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

