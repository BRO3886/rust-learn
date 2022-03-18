fn main() {
    let x = 5;
    println!("value of x {}", x);
    let x = x + 1;
    println!("value of x {}", x);

    {
        let x = x * 2;
        println!("value of x in inner scope {}", x);
    }

    println!("value of x after inner scope {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("value of constant {}", THREE_HOURS_IN_SECONDS);
}
