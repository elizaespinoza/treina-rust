fn main() {
    let notas: Vec<f32> = vec![10.0, 8.0, 6.5];

    println!("Notas: {:?}", notas);

    for nota in &notas {
        println!("Nota: {}", nota);
    }

    println!("Notas: {:?}", notas);
}
