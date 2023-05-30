// stack is used for static memory allocation and follows a LIFO order
// heap is used for dynamic memory allocation

fn main() {
    let mut uma_string = String::from("Testando ownership.");

    rouba(&mut uma_string);

    println!("{}", uma_string);
}

fn rouba(string: &mut String) {
    string.push_str(" Agora é minha!");

    println!("{}", string);
}
