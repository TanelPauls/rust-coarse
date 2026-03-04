// Collection Types
// Vectors
// UTF8
// Hash Maps

// Vectors allow to store more than one value in a single data structure,
//         that puts all the values next to eachother in memory.

fn main() {
    //let v: Vec<i32> = Vec::new();

    let mut v:Vec<i32> = Vec::new();
    let mut v: Vec<i32> = vec![1,2,3];

    
    v.push(5);
    v.push(6);
    v.push(7);

    println!("The numbers vec: {:?}", v);

    let vector: Vec<i32> = vec![1,2,3,4,5,6];

    let third: &i32 = &vector[2]; // direct indexing

    println!("Third element in vector: {}", third);

    let third: Option<&i32>= vector.get(2);
    match third {
        Some(third) => println!("Third element in with GET method is: {}", third),
        None => println!("There is no third element"),
    }

    // UTF8
    // 1
    let s: String = "whatever".to_string();
    // 2 
    let s: String = String::from("whatever");
    // 3 mutate the variable
    let mut s: String = String::from("foo");
    s.push_str(" bar"); 
    s.push('!'); // only for one character

    println!("String s: {s}");

    let s1: String = String::from("Hello");
    let s2: String = String::from(" World!");
    let s3: String = s1 + &s2; // Takes ownership of s1 and references s2

    println!("String s3: {s3}");

    // Hash map

    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name: String = String::from("Blue");
    let score: i32 = scores.get(&team_name).copied().unwrap_or(0);

    println!("Score: {score}");

    for (key, value) in &scores {
        println!("{key} : {value}");
    }

    
}

