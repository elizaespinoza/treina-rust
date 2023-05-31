fn main() {
    let notas: [f32; 4] = [6.5; 4];

    for indice in 0..notas.len() {
        println!("Nota {}: {}", indice, notas[indice]);
    }

    matriz();
}

fn matriz() {
    let matriz: [[f32; 3]; 2] = [[1.3, 2.6, 3.8], [4.1, 5.7, 6.4]];

    for linha in 0..matriz.len() {
        for coluna in 0..matriz[linha].len() {
            println!(
                "Elemento[{}][{}] = {}",
                linha, coluna, matriz[linha][coluna]
            );
        }
    }
}
