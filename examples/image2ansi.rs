use chafa::{image2ansi, Config};

fn main() {
    let ansi_output = image2ansi("examples/test.png", Config{ 
        dst_width : 33,
        dst_height : 16,
    });
    println!("{}", ansi_output);
}
