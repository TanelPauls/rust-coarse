//Variables and Mutability

//variables are immutable by default

fn main() {
    let a = 5;
    println!("The value of a is {}", a);
    // a = 10; <-- this produces an error 

    let mut b = 5;
    println!("The value of b is {}", b);
    b = 10; 
    println!("The value of b is {}", b);
}
