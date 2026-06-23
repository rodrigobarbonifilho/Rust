fn main() {
    // Se não tiver o tipo explicito o compilador não sabe bem
    // qual tipo será parseado a string
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Parsed guess: {guess}");

    // -=- Scalar Types -=-
    println!("-=- Scalar Types -=-");
    
    // O equivalente a tipos primitivos (Primitive Types)
    //
    // -- Integer Types
    println!("-- Integer Types");
    
    // +------------------------+--------+----------+
    // | Length                 | Signed | Unsigned |
    // +------------------------+--------+----------+
    // | 8-bit                  | i8     | u8       |
    // +------------------------+--------+----------+
    // | 16-bit                 | i16    | u16      |
    // +------------------------+--------+----------+
    // | 32-bit                 | i32    | u32      |
    // +------------------------+--------+----------+
    // | 64-bit                 | i64    | u64      |
    // +------------------------+--------+----------+
    // | 128-bit                | i128   | u128     |
    // +------------------------+--------+----------+
    // | Architecture-dependent | isize  | usize    |
    // +------------------------+--------+----------+
    //
    // Basicamente os signed possuem sinal e o outro não
    // o tradeoff para isso é que dados armazenados como signed
    // possuem um range de 256 possibilidades, porém seu maior valor é
    // menor comparado a um dado inteiro do tipo unsigned
    //
    // por exemplo, um número de 8-bits unsigned pode variar de
    // 0 a (2^8 - 1)  <=>    0 a 255
    // enquanto que um número de 8-bit signed pode variar de
    // -2^7 a 2^7 - 1 <=> -128 a 127
    //
    // Repare que ambos são de 8-bit, mas por o max de um u8 é 255 e o
    // de um i8 é 127 
    // 
    // Integer Literals in Rust
    // 
    // +-----------------+-------------+
    // | Number literals | Example     |
    // +-----------------+-------------+
    // | Decimal         | 98_222      |
    // +-----------------+-------------+
    // | Hex             | 0xff        |
    // +-----------------+-------------+
    // | Octal           | 0o77        |
    // +-----------------+-------------+
    // | Binary          | 0b1111_0000 |
    // +-----------------+-------------+
    // | Byte (u8 only)  | b'A'        |
    // +-----------------+-------------+
    //
    //  -- Floating-Point Types
    println!("-- Numeric Operations");
    
    // Rust possui dois tipos de ponto flutuante, f32 e f64
    // atualmente os processadores atuais processam os dois com 
    // basicamente a mesma capacidade, então compensa usar o f64
    // já que possui maior precisão.
    let x = 2.0;      // f64
    
    let y: f32 = 3.0; // f32

    println!("f64 (default): {x}");
    println!("f32:           {y}");

    // -=- Numeric Operations -=-
    println!("-=- Numeric Operations -=-");
    
    // addition
    let sum = 5 + 10;
    println!("Sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("Product: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Resulta em -1
    println!("Quotient: {quotient}");
    println!("Truncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("Remainder: {remainder}");
}
