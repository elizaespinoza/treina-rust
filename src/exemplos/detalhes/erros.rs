fn main() {
    match resultado() {
        Ok(msg) => println!("String de sucesso: {}", msg),
        Err(cod) => println!("Código de erro: {}", cod),
    }
}

fn resultado() -> Result<String, u8> {
    //Ok(String::from("Tudo deu certo"))
    Err(42)
}
