# Functions

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```
- We can define functions to have parameters
- In function signatures, you must declare the type of each parameter
- Function definitions are also statements; the entire preceding example is a statement in itself
- Statements do not return values. Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```
- here `y` evaluates to `4`, because the expression `{ x + 1 }` is evaluated and assigned to `y`.
- `x+1` is an expression, not a statement, thus it does not have a semi-colon.
-  If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. 

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    println!("the value of x is {}", x);
}
```
- Functions can return values to the code that calls them. 
- We don’t name return values, but we must declare their type after an arrow (->). 
- The return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. 
- This is why `five()` returns `5` and not `5` does not have a `;` as it is not a statement.
