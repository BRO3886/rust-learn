# Guessing game

```rust
use std::io;
```
- io library comes from the standard library (which is known as std)

```rust
let mut guess = String::new();
```
- creates a mutable variable called guess
- values are immutable by default, unless specified otherwise (using `mut`)

```rust
String::new()
```
- a function that returns a new instance of a String
- `String` is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
- `::` denotes associated function
- An associated function is implemented on a type, in this case String, rather than on a particular instance of a String. Some languages call this a static method.

```rust
io::stdin().read_line(&mut guess)
```
- If we hadn’t imported the `io` library with use `std::io` at the beginning of the program, we could still use the function by writing this function call as `std::io::stdin`.
- `stdin()` returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
- `read_line(&mut guess)` reads a line from the standard input and stores it in the variable `guess`.   
- `&mut guess` is a reference to a mutable variable. Where `&` represents a reference.
- References are immutable by default.

```rust
.expect("failed to read line");
```
- handling potential failure with `Result` type
- As mentioned earlier, `read_line` puts whatever the user enters into the string we pass to it, but it also returns a value—in this case, an io::Result.  The Result types are enums, and are often used with `matc`h, a conditional that makes it convenient to execute different code based on which variant an enum value is when the conditional is evaluated.
- Result’s variants are `Ok` or `Err`. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.
- If you don’t call expect, the program will compile, but you’ll get a warning

```toml
rand = "0.8.3"
```
- `rand` is a dependency of the project since rust does not have a built-in random number generator as a part of the standard library.

```rust
use rand::Rng;
...
let secret_number = rand::thread_rng().gen_range(1..101);
```
- `thread_rng()` returns a `ThreadRng` instance, which is a random number generator that is used to generate random numbers.

```rust
use std::cmp::Ordering;
...
match guess.cmp(&secret_num) {
        Ordering::Less => println!("too small"),
        Ordering::Equal => println!("you win"),
        Ordering::Greater => println!("too big"),
}
```
- A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. 
- Rust takes the value given to match and looks through each arm’s pattern in turn. 
- If the value given to match fits the pattern of the arm, the code in the arm is run.

```rust
let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
};
```
- Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess for example.
- `parse()` is a method on the `String` type. It takes a string and tries to convert it into a number.
- Instead of crashing the program if the user enters a string that can’t be converted into a number, we use the `match` expression to handle the error.
- When the number can be converted, we return the number, otherwise we `continue`.

```bash
cargo doc --open
```
- `cargo doc` generates documentation for the project as HTML.
- `--open` opens the documentation in the default browser.
