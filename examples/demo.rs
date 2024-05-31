use chafa::{Canvas, Config, PixelType, SymbolMap, Symbols};

fn main() {
    // see https://hpjansson.org/chafa/ref/chafa-using.html
 
    const PIX_WIDTH : i32 = 3;
    const PIX_HEIGHT : i32 = 3;
    const N_CHANNELS : i32 = 4;
    let pixels : [u8; (PIX_WIDTH * PIX_HEIGHT * N_CHANNELS) as usize] = [
        0xff, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff,
        0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
        0xff, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff
    ];

    let symbol_map = SymbolMap::new();
    symbol_map.add_by_tags(Symbols::ALL);

    let config = Config::new();
    config.set_geometry(23, 12);
    config.set_symbol_map(symbol_map);

    let canvas = Canvas::new(config);

    canvas.draw_all_pixels(PixelType::RGBA8_UNASSOCIATED,
                           &pixels,
                           PIX_WIDTH,
                           PIX_HEIGHT,
                           PIX_WIDTH * N_CHANNELS);

    let output : String = canvas.build_ansi();

    println!("{}", output);
}
