# Data Types

```bash
$ cargo run      
   Compiling data-types v0.1.0 (/Users/sidv/Desktop/projects/rust/rust-learn/5-data-types)
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ consider giving `guess` a type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `data-types` due to previous error
```
- rust is a statically typed language
- the compiler can usually infer the type of a variable
- but sometimes we need to tell the compiler what the type is by specfiying the type annotation
- in this case, we need to tell the compiler that the variable `guess` is of type `u32`

### Integer Type

| Length | Signed | Unsigned
| ------ | ------ | --------
| 8-bit  | i8     | u8
| 16-bit | i16    | u16
| 32-bit | i32    | u32
| 64-bit | i64    | u64
| 128-bit| i128   | u128
| arch   | isize  | usize

#### Integer Literals

| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

- In debug mode the compiler will panic and let you know about any integer overflow case (e.g. `i8` max value is 255)
- When compiled with the `--release` flag, the compiler will optimize the code and remove the overflow checks
- It then performs two's complement wrapping on the number (256 -> 0, 257 -> 1 and so on)

Other types: `f32`, `f64`, `char`, `bool`, 

### Tuple type
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
- tuples have a fixed length: once declared, they cannot grow or shrink in size.

Destructuring a tuple:
```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
let five_hundred = tup.0;
```
- `x`, `y` and `z` are now references to the first, second and third element of the tuple
- `x` is now 500, `y` is 6.4 and `z` is 1
- `five_hundred` is 500

### Array type
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```
- arrays are fixed in size
- stored in stack
Initialization:
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```
- `a` is now an array of 5 elements of type `i32`
```rust
let a = [3; 5];
```
- `a` is now an array of 5 elements of value 3

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
```
- rust will panic if you try to access an index that is out of bounds, instead of accessing an invalid memory location