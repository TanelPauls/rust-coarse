//primitive data types
// int, float, boo´l, char

// INTEGER
// Rust has signred (+ and -) and unsignred
//integer (only+) types of different sizes.

//i8, i16, i32, i64, i128: Signed integers.
//u8, u16, u32, u64, u128: Signed integers.

fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    //let y: u64 = -100; <- this will end up in compile error.
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);


// diff bet i32 (32 bits) and u64 (64 bits)
// range: i32 (-2**31 +2**31)
// i32 - 2147483647
// i64 - 9223372036854775807
    let mut e: i128 = 2147483647; // with i/32 it was too small and created an error, so I had to increase it
    let i: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);
    e = e + 1;
    println!("e + 1= : {}", e);


// Floats [Floating Point Types]
// f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

// Boolean Values: true, false

    let is_snowing: bool = true;

    println!("Is it snowing ? - {}", is_snowing);

// Character Type - char

    let letter: char = 'a';

    println!("First letter of the alphabey: {}", letter);


}