fn main() {
    let idade: i8 = 19;

    let tipo = if idade > 18 { "maior" } else { "menor" };

    let responsavel_autorizou: bool = true;

    println!("Você tem {} anos e é {} de idade.", idade, tipo);

    if idade > 18 {
        println!("Portanto pode ir pra balada.");
    } else if idade > 16 && responsavel_autorizou {
        println!("Portanto pode ir pra balada com assinatura do responsável.");
    } else {
        println!("Portanto não pode entrar na balada.");
    }
}
