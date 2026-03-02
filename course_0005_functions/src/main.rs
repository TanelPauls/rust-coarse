//Functions
// Entry point is always main
// all fucntions / variables should be written in snake_case

fn main() {
    hello_world();
    tell_height(189);
    human_id("Mike", 11, 189.0);
    let x = {
        let price = 5;
        let qty = 10;
        // return price * qty; <- this is not very "rusty"
        price * qty
    };
    println!("Result is: {}", x);

    let y  = add(4 ,6);
    println!("Value of y is : {}", y);
    println!("Value from function 'add' is: {}.", add(4,6));

    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);

}

fn hello_world() {
    println!("Hello, Rust!");
}

// you can insert input values

fn tell_height(height: u32){
    println!("My height is {} cm.", height);
}

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, my height is {} cd.", name, age, height);
}

// Expressions and Statemetns
// Expression: Anything that returns a value.
// Statement: Anything that does NOT return a value.
// Almost all statements in Rust end with ;

// Expression
// 5
// true & false
// add(3,4)
// if condition {value1} else {value2}
// ({code})


// functions returning values

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Statements
// Statement: Anything that does NOT return a value.
// Almost all statements in Rust end with ;
// let y = let x = 10;
// 1 Variable declarations: let x = 5;
// 2 Function definitions: fn foo() {}
// 3 Control flow statements: if condtion { /* code */} else { /* code */}, while condition { /* code */}, etc.
 
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}