// Primitive data types
// int, float, boolean, char

fn integer_testing() {
    // Integer
    // Rust has signed (+ and -) and unsigned integer (only +) types of different sizes.
    // i8, i16, i32, i64, i128: Signed integers.
    // u8, u16, u32, u64, u128: Unsigned integers.

    let i32_test_value: i32 = -42;
    let i64_test_value: u64 = 100;
    println!("Signed Integer: {}", i32_test_value);
    println!("Unsigned Integer: {}", i64_test_value);
    // diff bet i32 (32 bits) and i64 (64 bits)
    // range :
    // i32 -             2.147.483.647
    // i64 - 9.223.372.036.854.775.807
    let i32_max_value: i32 = 2147483647;
    let i64_max_value: i64 = 9223372036854775807;
    println!("Maximum value i32: {}", i32_max_value);
    println!("Maximum value i64: {}", i64_max_value);
}

fn float_testing() {
    // Floats [Floating Point Types]
    // f32, f64
    let pi: f64 = 3.14;
    println!("Value of PI: {}", pi)
}

fn boolean_testing() {
    // Boolean Values | true or false
    let is_snowing: bool = true;
    println!("Is it Snowing? {}", is_snowing);
}

fn char_testing() {
    // Character Type - char
    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter)
}

fn main() {
    let type_test: char = 'c';

    if type_test == 'i' {
        integer_testing();
    } else if type_test == 'f' {
        float_testing();
    } else if type_test == 'b' {
        boolean_testing();
    } else {
        char_testing();
    }
}
