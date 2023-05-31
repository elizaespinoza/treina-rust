fn main() {
    let conteudo = ler_arquivo(String::from(""));

    match &conteudo {
        Some(conteudo) => println!("{}", conteudo),
        None => println!("Não foi possível ler o arquivo"),
    };

    println!("{:?}", conteudo);
}

fn ler_arquivo(caminho: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
}
