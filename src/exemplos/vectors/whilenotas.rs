fn main() {
    let mut notas: Vec<f32> = vec![10.0, 8.0, 6.5];

    println!("Notas: {:?}", notas);

    while let Some(nota) = notas.pop() {
        println!("Valor removido = {}", nota);
    }

    println!("Notas: {:?}", notas);
}
