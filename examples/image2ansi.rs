use chafa::extra::image2ansi;

fn main() {
    let output = image2ansi("examples/test.png", (33, 16)).unwrap();
    println!("{}", output);
}
