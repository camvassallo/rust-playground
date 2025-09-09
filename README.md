# Rust Playground

This repository is a personal workspace for learning and experimenting with Rust. It's structured to allow me to create and run small, isolated code examples easily.

## How to Run an Example

This project uses Cargo's `bin` directory to manage runnable code examples. Each file in the `src/bin` directory is a separate program.

To run a specific file, use the following command:

```bash
cargo run --bin <filename>
```

**Note:** Do not include the `.rs` extension in the filename.

**Example:**
If your file is named `dynamic_arrays.rs`, you would run it with:

```bash
cargo run --bin dynamic_arrays
```

## Project Structure

* `src/bin/`: Contains all the individual code files and examples.

* `Cargo.toml`: Manages the project's dependencies. Any crates needed for your examples should be added here.

## Naming Convention

All file names in this project follow Rust's `snake_case` naming convention (e.g., `my_first_example.rs`).
```