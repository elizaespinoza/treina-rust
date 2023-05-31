fn main() {
    let dia: DiaDaSemana = DiaDaSemana::Sabado;
    println!("É final de semana? {}", e_final_de_semana(dia));

    let outrodia: DiaDaSemana = DiaDaSemana::Quarta;

    println!("É final de semana? {}", e_final_de_semana(outrodia));
}

enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

fn e_final_de_semana(dia: DiaDaSemana) -> bool {
    match dia {
        DiaDaSemana::Sabado | DiaDaSemana::Domingo => true,
        _ => false,
    }
}
