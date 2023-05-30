fn main() {
    let multiplicador: u8 = 3;

    println!("Estratégia: while");
    println!("Tabuada do {}", multiplicador);

    let mut contador: u8 = 0;

    while contador < 10 {
        contador += 1;

        // Pula a execução do bloco de código
        if contador == 5 {
            continue;
        }

        println!(
            "{} x {} = {}",
            multiplicador,
            contador,
            multiplicador * contador
        );
    }

    println!("Fim da tabuada do {}", multiplicador);

    println!("Estratégia: loop");

    println!("Tabuada do {}", multiplicador);

    let mut contador: u8 = 0;

    loop {
        contador += 1;
        println!(
            "{} x {} = {}",
            multiplicador,
            contador,
            multiplicador * contador
        );

        if contador == 10 {
            break;
        }
    }

    println!("Fim da tabuada do {}", multiplicador);
}
