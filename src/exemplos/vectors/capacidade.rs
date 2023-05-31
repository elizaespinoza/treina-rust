fn main() {
    //let mut notas: Vec<f32> = vec![10.0, 8.0, 6.5];

    let mut notas: Vec<f32> = Vec::with_capacity(8);

    println!("Notas: {:?}", notas);

    println!("Capacidade: {}, Tamanho: {}", notas.capacity(), notas.len());

    notas.push(10.0);
    notas.push(8.8);

    println!("Notas: {:?}", notas);

    println!("Capacidade: {}, Tamanho: {}", notas.capacity(), notas.len());

    notas.push(9.6);
    notas.push(9.9);

    println!("Notas: {:?}", notas);

    println!("Capacidade: {}, Tamanho: {}", notas.capacity(), notas.len());

    println!(
        "Nota 7: {}",
        match notas.get(8) {
            Some(nota) => *nota,
            None => 0.0,
        }
    );
}
