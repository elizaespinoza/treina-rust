fn main() {
    let conteudo = ler_arquivo(String::from(""));

    if let None = conteudo {
        println!("Não foi possível ler o arquivo");
    } else {
        println!("{}", conteudo.unwrap());
    }
}

fn ler_arquivo(caminho: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
}
