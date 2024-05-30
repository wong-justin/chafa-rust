use chafa::{image2ansi, Config, Symbols};

fn main() {
    let output = image2ansi("examples/test.png", Config{ 
        cols: 33,
        rows: 16,
        quality: 0.9,
        symbols: Symbols::BLOCK | Symbols::BRAILLE,
    }).unwrap();
    println!("{}", output);
}
