fn main() {
    cores();
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn cores() {
    let cor: Color = Color::RgbColor(54, 77, 1);

    let cor2: Color = Color::CmykColor {
        cyan: 100,
        magenta: 50,
        yellow: 70,
        black: 255,
    };

    println!(
        "Cor = {}",
        match cor2 {
            Color::Red => "Vermelho",
            Color::Green => "Verde",
            Color::Blue => "Azul",
            Color::RgbColor(0, 0, 0)
            | Color::CmykColor(
                cyan: _,
                magenta: _,
                yellow: _,
                black: 255,
            ) => "Preto",
            Color::RgbColor(_, _, _) => "RGB desconhecido",
            Color::CmykColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: _,
            } => "CMYK desconhecido",
        }
    );
}
