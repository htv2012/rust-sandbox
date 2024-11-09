
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

* **Return Value**: The result given by a function upon completion. It can
  be explicitly defined or implicitly returned as unit type in case of
  unit functions.
* **Borrowing**: Concept in Rust which ensures efficient memory usage by
  safely lending values to other parts of the code without taking
  ownership away from their original scopes.
* **Panic**: A special call syntax in Rust used to stop all execution in a
  program; it's not commonly used in production code but can be helpful
  during development or for certain error handling scenarios.
* **Control Flow**: The order in which code is executed based on conditions
  and loops. In Rust, control flow includes if, else, match, and looping
  constructs like for or while.
* **Enumerator (Enum)**: A data type representing a set of values where each
  value represents a distinct case. An example in the transcript is the
  Option<T> enum with cases Some(T) and None.
* **Move**: In Rust, move occurs when ownership of a variable is transferred
  from one scope to another without any borrowing mechanism being used.
  This results in the original variable becoming invalid.
* **Copy**: A special trait in Rust which allows values of certain types
  (e.g., integers and booleans) to be copied instead of moved or
  borrowed when assigned or passed as arguments.
* **Vector**: A dynamic array data structure provided by Rust's standard
  library, used for storing a variable number of elements efficiently.
  It can grow or shrink in size during runtime.
* **Shadowing**: In Rust, the ability to redefine a variable with a new
  value within the same scope hiding the original definition but not
  invalidating it. This is useful when changing types locally without
  affecting other parts of the code.

# Key Terms
* **Struct**: A keyword used to organize similar data in a structure. It
  is like an object in JavaScript or Python dictionary where you are
  organizing data in a structured way.
* **Field**: The values of the struct, such as first name and last name
  string for person struct.
* **Debug**: An attribute that allows printing the whole struct instead
  of specific fields.
* **Type**: The kind of value each field can hold, such as string or
  unsigned integer 8 bits in size (u8).
* **Instance**: A created struct with data in its fields, like Fredo
  equals person with first name Sanchez and age H25.
* **Option**: Represents the absence of a value or a specific type that
  could be (for example) an unsigned integer for eight bits in size (u8)
  or none.
* **Implementation**: A keyword used to extend struct by adding
  functions and associated code.
* **Associated Function**: A function that doesn't require self,
  allowing easy creation of a user instance with new constructor.
* **Constructor**: Automates tedious repetitive tasks when creating
  instances, like setting the active field to true in user struct.
* **Immutable**: Cannot be changed after initialization, such as new
  user being immutable by default.

# Key Terms
* **String**: A sequence of characters, typically used to represent text. In Rust, there are two primary types of strings: string slices (&str) and strings (String).
* **String slice**: A reference to a sequence of characters in memory. It is immutable and has a fixed size. Represented as &str.
* **String (this is not repeated!)**: A growable, owned sequence of characters. It is mutable and its size can change during runtime. Represented as String.
* **Vector**: A collection of items that can be of any type. It is similar to arrays or lists in other languages. In Rust, vectors are represented as Vec<T>, where T is the type of elements contained within the vector.
* **Immutable**: A value that cannot be changed after it has been created. Strings slices and vectors can be immutable.
* **Mutable** - A value that can be changed after it has been created. Only strings and mutable vectors can be mutable.
* **Borrowing**: The process of temporarily accessing a resource without taking ownership. In Rust, borrowing is used to allow multiple references to the same data without violating memory safety rules.
* **Ownership**: The concept that a value can only have one owner at a time in Rust. When a value is transferred or dropped, its previous owner loses access to it.
* **Slice**: A view into a contiguous sequence of elements, such as an array, string, or vector. It has a fixed size and does not own the data it points to. Represented as [T].
* **Borrowing and Lifetimes**: Borrowing is a mechanism in Rust that allows multiple references to the same data without violating memory safety rules. Lifetimes ensure that borrowed references remain valid for as long as they are needed, preventing dangling pointers or use-after-free errors.
* **Mutable References**: A mutable reference is a reference to a value that can be changed during its lifetime. In Rust, only one mutable reference can exist at any given time for a particular piece of data, ensuring memory safety and preventing race conditions.

# Keywords
* **Enum**: An enum, short for enumerator, is a data type in Rust that
  allows you to define a set of possible values. It's used to create
  custom types that represent distinct variants or cases. Enums are
  powerful because they enable you to encapsulate related values and
  provide exhaustive matching capabilities through the match keyword.
* **Variant**: A variant is a specific case within an enum. Each enum can
  have multiple variants, which may optionally contain associated data.
  For example, in the text provided, there are enums like Shape, with
  two variants: Circle and Square. The Circle variant has an associated
  radius value, while the Square variant does not carry any additional
  data.
* **Match** keyword: The match keyword is a powerful control flow construct
  that enables exhaustive pattern matching against enums or other
  values. It checks each case to find the first match, making it an
  excellent way to handle different enum variants in Rust. The text
  demonstrates using match with enums like Shape, WineRegion, and more.
* **Option**: The Option type is a built-in enum provided by Rust that
  represents either the presence or absence of a value, denoted as
  Some(value) or None. It's often used to handle nullable values in
  other languages. The text explores using Option, including its
  unwrapping methods like unwrap().
* **Sum** type: A sum type is an enumeration that can hold a variant with
  associated data of some type D, allowing you to create instances
  containing different types or combinations of data. In Rust, enums are
  examples of sum types because they let you define multiple variants,
  each potentially carrying distinct information. The text demonstrates
  this concept through the Shape enum and its associated data (radius
  for circles and side length for squares).
* **Associated** functions/methods: In Rust, it's possible to extend enums
  with implementation blocks that contain associated functions or
  methods, just like structs. This enables you to add functionality
  specific to the enum type directly within the enum definition. The
  text showcases this by implementing a format_size() method for the
  FileSize enum.
* **Vectors**: A vector is a growable array-like data structure in Rust that
  can hold multiple values of the same type. In the context of enums,
  vectors are useful when you want to store and manipulate collections
  of related enum instances or variants. The text demonstrates using
  vectors with enums by defining a Shapes vector containing different
  shapes (circles and squares).
* **Exhaustive** matches: Exhaustive matching is the practice of handling
  all possible cases when working with enums, ensuring that every
  variant has been accounted for in your code. Rust's type system helps
  enforce exhaustiveness by requiring you to handle each enum variant or
  use a catch-all pattern (_) if needed. The text highlights this
  concept through examples like the taste() function dealing with wine
  grape variants.
* **Non-exhaustive patterns**: When working with enums, non-exhaustive
  patterns refer to cases where not all possible enum variants are
  explicitly handled in a match statement or conditional expression.
  Rust's compiler will warn you about missing arms (unhandled variants)
  unless you use the catch-all pattern _ or explicitly mark your enum as
  non-exhaustive using the ... syntax. The text demonstrates this
  concept through examples like the wine grape enumerator and its
  associated functions.


# Some helpful commands

```bash
# Create a new project
cargo new my_project

# Run it
cd my_project
cargo run

# Edit
vim src/main.rs

# Format
cargo fmt --all

# Documentation
cargo doc --open

# Build
cargo build

# Clean
cargo clean
```

