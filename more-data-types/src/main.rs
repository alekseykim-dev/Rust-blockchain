fn main() {
    // Signed Integers
    let small_signed: i8 = -128; 
    let medium_signed: i16 = -32768; 
    let regular_signed: i32 = -2_147_483_648; 
    let large_signed: i64 = -9_223_372_036_854_775_808; 
    let very_large_signed: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728; 

    // Unsigned Integers
    let small_unsigned: u8 = 255; 
    let medium_unsigned: u16 = 65_535; 
    let regular_unsigned: u32 = 4_294_967_295; 
    let large_unsigned: u64 = 18_446_744_073_709_551_615; 
    let very_large_unsigned: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; 

    // Floating-Point Numbers
    let small_float: f32 = 3.14; //
    let large_float: f64 = 2.718281828459045; 

    // Boolean
    let is_rust_amazing: bool = true;

    // Characters
    let letter: char = 'R'; // Single character
    let emoji: char = '🚀'; // Unicode character

    println!("--- Signed Integers ---");
    println!("i8: {}", small_signed);
    println!("i16: {}", medium_signed);
    println!("i32: {}", regular_signed);
    println!("i64: {}", large_signed);
    println!("i128: {}", very_large_signed);

    println!("\n--- Unsigned Integers ---");
    println!("u8: {}", small_unsigned);
    println!("u16: {}", medium_unsigned);
    println!("u32: {}", regular_unsigned);
    println!("u64: {}", large_unsigned);
    println!("u128: {}", very_large_unsigned);

    println!("\n--- Floating-Point Numbers ---");
    println!("f32: {}", small_float);
    println!("f64: {}", large_float);

    println!("\n--- Boolean ---");
    println!("Is Rust amazing? {}", is_rust_amazing);

    println!("\n--- Characters ---");
    println!("Letter: {}", letter);
    println!("Emoji: {}", emoji);

    // Tuples
    let person: (&str, u8, bool) = ("Alice", 30, true);
    println!("\n--- Tuple ---");
    println!("Name: {}, Age: {}, Active: {}", person.0, person.1, person.2);


    
    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("\n--- Array ---");
    println!("Array: {:?}, First Element: {}", numbers, numbers[0]);

    // Slices
    let slice = &numbers[1..4];
    println!("Slice of Array: {:?}", slice);

    // Vectors
    let mut vec = vec![10, 20, 30];
    vec.push(40);
    println!("\n--- Vector ---");
    println!("Vector: {:?}, Length: {}", vec, vec.len());

    // Strings
    let string_literal = "Hello, Rust!"; // String slice
    let mut owned_string = String::from("Rust is awesome!");
    owned_string.push_str(" 🚀");
    println!("\n--- Strings ---");
    println!("String Literal: {}", string_literal);
    println!("Owned String: {}", owned_string);

    // Enum
    let current_direction = Direction::North;
    println!("\n--- Enum ---");
    match current_direction {
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Heading South!"),
        Direction::East => println!("Heading East!"),
        Direction::West => println!("Heading West!"),
    }

    // Using all these types in a function
    calculate(small_signed, medium_unsigned, small_float, is_rust_amazing, letter);
}

// Function using multiple data types
fn calculate(i: i8, u: u16, f: f32, b: bool, c: char) {
    println!("\n--- Function Output ---");
    println!("Received i8: {}", i);
    println!("Received u16: {}", u);
    println!("Received f32: {}", f);
    println!("Received bool: {}", b);
    println!("Received char: {}", c);

    // Example Calculation
    let result = (i as f32 + u as f32) * f;
    println!("Calculated Result: {}", result);
}

// Enum to represent directions
enum Direction {
    North,
    South,
    East,
    West,
}
