// convience abstracted functions

use image;

// very similar to: https://hpjansson.org/chafa/ref/chafa-using.html
pub fn image2ansi<P>(path: P, (cols, rows): (u32, u32)) -> Result<String, image::ImageError>
where
    P: AsRef<std::path::Path>,
{
    // --- IMAGE DATA --- //

    let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::open(path)?.to_rgba8();

    // chafa has variations of 8bit rgb channels...
    //
    // RGBA8_PREMULTIPLIED, RGBA8_UNASSOCIATED, BGRA_PREMULTIPLIED, ...
    //
    // while the image crate accepts more channel types and bit depths:
    //
    // L8 (8bit luminance, ie. grayscale)
    // La8 (8bit luminance with transparency)
    // Rgb8 (etc)
    // Rgba8
    // L16
    // La16
    // Rgb16
    // Rgba16
    // Bgr8
    // Bgra8
    //
    // I'll just force rgba8 for now

    let num_channels = 4; // since everything is rgba8 // = img.color().channel_count().into();
    let src_width: i32 = img.width().try_into().unwrap();
    let src_height: i32 = img.height().try_into().unwrap();
    let pixels: Vec<u8> = img.into_raw();

    // --- CHAFA CONFIG --- //

    let symbol_map = crate::SymbolMap::new();
    symbol_map.add_by_tags(crate::Symbols::BLOCK);

    let config = crate::Config::new();
    config.set_geometry(cols as i32, rows as i32);
    config.set_symbol_map(symbol_map);
    config.set_work_factor(1.0);

    let canvas = crate::Canvas::new(config);

    // --- OUTPUT --- //

    canvas.draw_all_pixels(
        crate::PixelType::RGBA8_UNASSOCIATED,
        &pixels,
        src_width,
        src_height,
        src_width * num_channels,
    );

    let result = canvas.build_ansi();
    return Ok(result);
}

// TODO potential functions:
//
// fn image2pixels - when you need image data but don't want to display yet
// fn pixels2ansi - when frames come from other sources, like reading video frames
// fn images2ansi - keep the canvas around for continuous use, eg. animated gifs
