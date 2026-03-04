// Shadowing

// Shadowing is not the same as marking a variable as mut

fn main() {
    let x = 5; // This variable will be shadowed in next line -result 5

    let x = x+1; // -result 6

    // x = 10; <- this is still an error.

    {
        let x = x * 2; // -result 12
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
