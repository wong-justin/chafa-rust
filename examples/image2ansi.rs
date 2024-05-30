use chafa::{image2ansi, Config, Symbols};

fn main() {
    let ansi_output = image2ansi("examples/test.png", Config{ 
        dst_width : 33,
        dst_height : 16,
    });
    println!("{}", ansi_output);
        quality: 0.9,
        symbols: Symbols::BLOCK | Symbols::BRAILLE,
}
