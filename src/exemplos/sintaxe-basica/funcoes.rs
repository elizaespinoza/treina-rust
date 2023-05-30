fn soma(a: i32, b: i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn main() {
    println!("A soma de 1 e 2 é {}", soma(1, 2));
}
