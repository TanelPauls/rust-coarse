// Error Handling techniques ( 2 approaches )

// option 1

/* enum Option<T>{ // Define the generic Option type
    Some(T), // represents a value
    None // represents no value
} */

fn divide(numerator: f64, denominator: f64) ->Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// option 1

/* enum Result<T, E>{ // Define the generic Result type
    Ok(T), // represents a value
    Err(E), // represents an error
} */

fn divide_2(numerator: f64, denominator: f64) ->Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by 0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}



fn main() {
    //option 1
    let result = divide(1.0, 0.0);
    match result {
        Some(x) => println!("Result : {}", x),
        None => println!("Error, denominator is 0!")
    }

    let result = divide(1.0, 10.0);
    match result {
        Some(x) => println!("Result : {}", x),
        None => println!("Error, denominator is 0!")
    }

    // option 2

    let result = divide_2(1.0, 0.0);
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    let result = divide_2(10.0, 2.0);
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
    
}
