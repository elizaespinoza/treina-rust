fn main() {
    let linguagem = "Rust";

    let proposito = match linguagem {
        "Rust" => "Segurança e velocidade",
        "Python" => "Legibilidade e produtividade",
        "C" => "Desempenho",
        "C++" => "Desempenho e flexibilidade",
        "Java" => "Portabilidade",
        "C#" => "Portabilidade e produtividade",
        _ => "Linguagem desconhecida",
    };

    println!("O propósito do {} é {}", linguagem, proposito);
}
