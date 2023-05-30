fn main() {
    for num_linguas in 1..=10 {
        println!(
            "{}: {}",
            num_linguas,
            match num_linguas {
                1 => "uma língua",
                2 | 3 => "poucas línguas",
                4..=6 => "algumas línguas",
                _ if num_linguas % 2 == 0 => "um número par de línguas",
                _ => "muitas línguas",
            }
        );
    }
}
