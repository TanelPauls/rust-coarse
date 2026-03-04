// Conditions

// If else  [ if conditions]

fn main() {
    let age: u16 = 17;

    if age >= 18 {
        println!("You can drive a car!");
    } else {
        println!("You can't drive a car!");
    }

    // Multiple conditions with else if
    
    let number = 6;
    if number %4 == 0 {
        println!("Number divisible by 4");
    } else if number %3 == 0 {
        println!("Number divisible by 3");
    } else if number %2 == 0 {
        println!("Number divisible by 2");
    } else {
        println!("Number not divisible by 4,3,2");
    }

    // Using if in a let statement

    let condition = true;
    let number = if condition {5} else {10};
    // let number = if condition {5} else {"ten"}; <- this will produce a type error.
    println!("Number : {number}");
}

