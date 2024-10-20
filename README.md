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

# Functions

* Return Value: The result given by a function upon completion. It can
  be explicitly defined or implicitly returned as unit type in case of
  unit functions.
* Borrowing: Concept in Rust which ensures efficient memory usage by
  safely lending values to other parts of the code without taking
  ownership away from their original scopes.
* Panic: A special call syntax in Rust used to stop all execution in a
  program; it's not commonly used in production code but can be helpful
  during development or for certain error handling scenarios.
* Control Flow: The order in which code is executed based on conditions
  and loops. In Rust, control flow includes if, else, match, and looping
  constructs like for or while.
* Enumerator (Enum): A data type representing a set of values where each
  value represents a distinct case. An example in the transcript is the
  Option<T> enum with cases Some(T) and None.
* Move: In Rust, move occurs when ownership of a variable is transferred
  from one scope to another without any borrowing mechanism being used.
  This results in the original variable becoming invalid.
* Copy: A special trait in Rust which allows values of certain types
  (e.g., integers and booleans) to be copied instead of moved or
  borrowed when assigned or passed as arguments.
* Vector: A dynamic array data structure provided by Rust's standard
  library, used for storing a variable number of elements efficiently.
  It can grow or shrink in size during runtime.
* Shadowing: In Rust, the ability to redefine a variable with a new
  value within the same scope hiding the original definition but not
  invalidating it. This is useful when changing types locally without
  affecting other parts of the code.

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

