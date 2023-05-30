fn main() {
    let inteiro: i32 = 300;
    println!(
        "Variavel = {}, tamanho = {} bytes",
        inteiro,
        std::mem::size_of_val(&inteiro)
    );

    let decimal: f32 = 3.14;
    println!(
        "Variavel = {}, tamanho = {} bytes",
        decimal,
        std::mem::size_of_val(&decimal)
    );

    let booleano: bool = false;
    println!(
        "Variavel = {}, tamanho = {} bytes",
        booleano,
        std::mem::size_of_val(&booleano)
    );

    let letra: char = 'A';
    println!(
        "Variavel = {}, tamanho = {} bytes",
        letra,
        std::mem::size_of_val(&letra)
    );
}
