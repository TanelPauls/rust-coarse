// Compound Data Types
// arrays, tuples, slices and string (slice string)


// Arrays 
// all types in array must be the same type [1,2,3,4,5]

fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5123];
    println!("Number array: {:?}", numbers);

    //let mix = [1,2 "apple", true]; <- cat do that
    //println!("Number array: {:?}", mix);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits array: {:?}", fruits);
    println!("Fruits array: {}", fruits[0]);
    println!("Fruits array: {}", fruits[1]);
    println!("Fruits array: {}", fruits[2]);


    //Tuples
    let human1: (&str, i32, bool) = ("Alice", 30, false);
    let human2 = ("Alice", 30, false);
    let human3: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human1 Tuple: {:?}", human1);
    println!("Human2 Tuple: {:?}", human2);
    println!("Human3 Tuple: {:?}", human2);

    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);

    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices:[1,2,3,4,5]

    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices:&[&str] = &["Lion","Elephant","Crocodile"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices:&[&String] = &[&"IT".to_string(),&"Harry Potter".to_string(),&"ZEN".to_string()];
    println!("Book Slice: {:?}", book_slices);

    // Strings Vs String Slices (&str)
    // Strings are growable (growable, mutable, owned string type)

    let mut stone_cold: String = String::from("River, ");
    println!("Stone Cold Says: {}", stone_cold);

    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (String Slice)
    // this is "borrowing" from real string, which is good for memort efficiency

    let string: String = String::from("Hello, World!");
    let slice: &str = &string;
    let slice2: &str = &string[0..5];
    println!("Slice Value: {}", slice);
    println!("Slice2 Value: {}", slice2);
}

// this produces error, since its "outside scope"

//fn print() {
//    println!("Slice: {}", slice);
//}

