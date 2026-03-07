// Compound Data Types
// arrays, tuples, slices and strings (slice string)

fn main() {
    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);
    // Sem o :? dá erro, devido aos tipos de formatters
    // Default Formatter
    // Debbuger Formatter
    // Aparentemente vou ver mais tarde sobre isso

    // let mix = [1, 2, "apple", true]; // This doesnt work
    // println!("Mix Array: {?:}", mix);=>

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2nd element: {}", fruits[1]);
    println!("Fruits Array 3rd element: {}", fruits[2]);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple: (&str, u32, bool, [i32; 5]) = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices: [1, 2, 3, 4, 5]
    let numbers_array: [u32; 5] = [1, 2, 3, 4, 5];
    let number_slices: &[u32] = &numbers_array[1..3];
    println!("Number Slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Girafa", "Elefante"];
    println!("Animal Slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"Book1".to_string(), &"Book 2".to_string()];
    println!("Book Slices: {:?}", book_slices);

    // Strings vs String Slices (&str)
    // Strings [ growable, mutable, owned string type ]
    let mut stone_cold: String = String::from("Hello, ");
    stone_cold.push_str("World!");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (String Slice)
    let string: String = String::from("Hello, World!!");
    let string_slice: &str = &string[0..5];
    println!("String Sliced: {}", string_slice);
    print(string_slice);
}

fn print(string_slice: &str) {
    println!("{}", string_slice);
}
