fn escopo() {
    let x = 10;
    {
        let y = 20;
        println!("x: {}, y: {}", x, y);
        println!("x: {}", x);
    }

    println!("x: {}", x);
}

fn main() {
    escopo();

    let essa_string: &'static str = "Hello, world!";
    println!("{}", essa_string);
}
