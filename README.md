# Rust Undefined Behavior Example

This repository demonstrates a common source of undefined behavior in Rust: accessing memory after the owning object has been dropped.  Specifically, it shows how using a raw pointer to a vector after the vector has been deallocated leads to unpredictable results.

The `bug.rs` file contains the buggy code.  The `bugSolution.rs` file offers a corrected version using safe Rust techniques.

## How to Reproduce

1. Clone this repository.
2. Navigate to the repository's directory.
3. Run `rustc bug.rs && ./bug` (The program may crash or produce unexpected output).
4. Run `rustc bugSolution.rs && ./bugSolution` (This will output a corrected behavior).

## Understanding the Issue

The core problem is that the raw pointer `ptr` retains a reference to memory that is no longer valid after `drop(v);`. Accessing this memory after deallocation causes undefined behavior—the program might crash, produce incorrect results, or appear to work correctly in some cases, making it exceptionally difficult to debug.