// ownership, borrowing and references

//Ownership

//every cakye has a single owner. Every variable 
// has one value and it is its sole owner.


//owmership rules

// 1. each value in Rust has a variable that's its owner.
// 2. There can be only one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

//example: each value in Rust has a variable that's its owner:

fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);

    let s2 = s1;


    // example 2: there can only be one owner at a time.
    //println!("{}", s1); <- produces and error
    println!("{}", s2);

    
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

// example 3: When the owner goes out of scope, the value will be dropped.
/* fn printLost(s: &string) {
    println!("{}", &s1);
} */
