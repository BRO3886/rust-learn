# Control Flow

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```
- the above is an example of a `if` statement
- the expressions should always evaluate to a `bool`
- for example, ```if number {``` would cause a compilation error, since number is not a `bool`

```rust
fn main() {

    let condition = true;

    let number = if condition { 5 } else { "six" };
}
```
- we can use expressions to assign values to variables in the `if` statement
- the above code will not compile since the expressions under if and else are of different types
- instead of `"six"`, we can use `6`, then the compiler will infer the type of the variable correctly and the code will compile

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```
- the above is an example of a `loop` statement
- the loop will run forever until we break out of it
- Rust also provides a way to break out of a loop using code. You can place the `break` keyword within the loop to tell the program when to stop executing the loop.

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
```
- The above is an example of nested loops
- If you have loops within loops, break and continue apply to the innermost loop at that point. 
- You can optionally specify a loop label on a loop that we can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. 

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```
- in rust you can also return a value with break


```rust
let a = [10, 20, 30, 40, 50];
let mut index = 0;

while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
}

let a = [10, 20, 30, 40, 50];
for element in a {
    println!("the value is: {}", element);
}

 for number in (1..4).rev() {
    println!("{}!", number);
}
println!("LIFTOFF!!!");
```
- the above is an example of a `while` and `for` loop
