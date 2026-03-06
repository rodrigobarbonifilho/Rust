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
    // println!("Mix Array: {?:}", mix);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2nd element: {}", fruits[1]);
    println!("Fruits Array 3rd element: {}", fruits[2]);

    // Tuples
    
}
