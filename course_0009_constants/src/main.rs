// Constants

// Constants are by default immutable and you cant change it to mutable

// Constants should be in capital letters and add type annotations

fn main() {
    // const mut y = 10; <-- cant do that, constans are not mutable
    const Y: i32 = 10;
    println!("The value of Y is {}", Y);
    println!("The value of Pi is {}", PI);
    println!("The value of THREE_HOURS_IN_SECONDS is {}", THREE_HOURS_IN_SECONDS);
}

// we can declare constants outside main

const PI: f64 = 3.141592653;
const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
