fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

        std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    
    // Em tempo de compilação o Rust verifica se o indice informado é
    // maior ou igual ao tamanho do array, e fica em panick caso o seja
    // em outras linguagens de baixo nível provavelmente seria acessado
    // um endereço de memoria fora do planejado, resultando possivelmente
    // em lixo de memória
}
