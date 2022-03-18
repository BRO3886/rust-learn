# Cargo

Cargo is Rust’s build system and package manager. 

To build a project:
```bash
cargo build
```

To build and run:
```bash
cargo run
```
- If you don't change your code it will run the same binary again

To check if program compiles:
```bash
cargo check
```
- cargo check is much faster than cargo build, because it skips the step of producing an executable. 
- If you’re continually checking your work while writing the code, using cargo check will speed up the process! As such, many Rustaceans run cargo check periodically as they write their program to make sure it compiles. Then they run cargo build when they’re ready to use the executable.

> The executables created from the above commands would live under target/debug

To build for release
```bash
cargo build --release
```
- compiles with optimizations
- `target/release/`
