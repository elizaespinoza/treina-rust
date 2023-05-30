fn main() {
    let idade: i8 = 19;

    let responsavel_autorizou: bool = true;

    let maior_idade: bool = idade > 18;

    let classificação = if maior_idade { "maior" } else { "menor" };

    println!("Voce e {} de idade", classificação);

    if maior_idade {
        println!("Portanto pode ir pra balada.");
    } else if idade > 16 && responsavel_autorizou {
        println!("Portanto pode ir pra balada com assinatura do responsável.");
    } else {
        println!("Portanto não pode entrar na balada.");
    }
}
