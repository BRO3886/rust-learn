# Variables

```rust
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("value of x {}", x);
4 |     x = 12;
  |     ^^^^^^ cannot assign twice to immutable variable
```
- variables are immutable by default
- cannot assign a value to variable x twice
- `mut` keyword is used to make a variable mutable

```rust
fn main() {
    let mut x = 5;
    println!("value of x {}", x);
    x = 12;
    println!("value of x {}", x);
}
```
- there are various tradeoffs between mutability and immutability
- For example, in cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances. 
- With smaller data structures, creating new instances and writing in a more functional programming style may be easier to think through, so lower performance might be a worthwhile penalty for gaining that clarity.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
- constants are defined with the `const` keyword
- constants are immutable by default
- you aren’t allowed to use `mut` with constants
- the type of a constant must be annotated
- Rust’s naming convention for constants is to use all uppercase with underscores between words

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}
```
- first binds the variable x to the value 5
- then binds **shadows** x by repeating `let` statement and binds it to the value 6
- then within the inner scope, x is shadowed again and multiplied by 2
- when the inner scope ends, x is still 6
- Shadowing is different than `mut` in the sense that the variable is still immutable and apply some transformations to it

> {} are boundaries for lexical scopes in rust