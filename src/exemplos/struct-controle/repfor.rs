fn main() {
    let multiplicador: u8 = 3;

    println!("Estratégia: for");

    println!("Tabuada do {}", multiplicador);

    for num in 1..=10 {
        println!("{} x {} = {}", multiplicador, num, multiplicador * num);
    }

    println!("Fim da tabuada do {}", multiplicador);
}
