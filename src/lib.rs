mod chafa;

pub use chafa_sys::*;

use image::ImageBuffer;

// hacky bitflags
pub mod Symbols {
    pub const NONE : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_NONE;
    pub const SPACE : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_SPACE;
    pub const SOLID : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_SOLID;
    pub const STIPPLE : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_STIPPLE;
    pub const BLOCK : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_BLOCK;
    pub const BORDER : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_BORDER;
    pub const DIAGONAL : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_DIAGONAL;
    pub const DOT : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_DOT;
    pub const QUAD : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_QUAD;
    pub const HHALF : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_HHALF;
    pub const VHALF : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_VHALF;
    pub const HALF : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_HALF;
    pub const INVERTED : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_INVERTED;
    pub const BRAILLE : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_BRAILLE;
    pub const TECHNICAL : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_TECHNICAL;
    pub const GEOMETRIC : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_GEOMETRIC;
    pub const ASCII : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_ASCII;
    pub const ALPHA : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_ALPHA;
    pub const DIGIT : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_DIGIT;
    pub const ALNUM : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_ALNUM;
    pub const NARROW : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_NARROW;
    pub const WIDE : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_WIDE;
    pub const AMBIGUOUS : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_AMBIGUOUS;
    pub const UGLY : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_UGLY;
    pub const LEGACY : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_LEGACY;
    pub const SEXTANT : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_SEXTANT;
    pub const WEDGE : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_WEDGE;
    pub const LATIN : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_LATIN;
    pub const IMPORTED : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_IMPORTED;
    pub const EXTRA : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_EXTRA;
    pub const BAD : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_BAD;
    pub const ALL : i32 = chafa_sys::ChafaSymbolTags_CHAFA_SYMBOL_TAG_ALL;
}

pub struct Config {
    pub cols: u32, // terminal cols
    pub rows: u32, // terminal rows
    pub quality: f32, // work_factor, 0-1
    pub symbols: i32 // chafa bitflags
    // font_ratio: f32,
}


// TODO potential functions:
//
// fn image2pixels - when you need image data but don't want to display yet
// fn pixels2ansi - when frames come from other sources, like reading video frames
// fn images2ansi - keep the canvas around for continuous use, eg. animated gifs


// very similar to: https://hpjansson.org/chafa/ref/chafa-using.html
pub fn image2ansi<P>(path: P, config: Config) -> Result<String, image::ImageError>
    where P: AsRef<std::path::Path> 
{

        // --- IMAGE DATA --- //

        // how to get pixel data from image files in rust?
        // image crate seems like most popular option
        // https://crates.io/search?q=image%20pixels
        // https://docs.rs/crate/image/0.23.14
        // https://docs.rs/image/0.23.14/image/
        // MIT license, many millions of downloads, created ~2021
        // 8.75MB feels awful heavy though
        // 
        let img : ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::open(path)?.to_rgba8();

        // chafa has variations of 8bit rgb channels...
        //
        // CHAFA_PIXEL_RGBA8_PREMULTIPLIED
        // CHAFA_PIXEL_BGRA8_PREMULTIPLIED
        // CHAFA_PIXEL_ARGB8_PREMULTIPLIED
        // CHAFA_PIXEL_ABGR8_PREMULTIPLIED
        // CHAFA_PIXEL_RGBA8_UNASSOCIATED
        // CHAFA_PIXEL_BGRA8_UNASSOCIATED
        // CHAFA_PIXEL_ARGB8_UNASSOCIATED
        // CHAFA_PIXEL_ABGR8_UNASSOCIATED
        // CHAFA_PIXEL_RGB8
        // CHAFA_PIXEL_BGR8
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
        let src_width : i32 = img.width().try_into().unwrap();
        let src_height : i32 = img.height().try_into().unwrap();
        let pixels : Vec<u8> = img.into_raw();

        // or test with made-up pixel buffer
        // const PIX_WIDTH : i32 = 2;
        // const PIX_HEIGHT : i32 = 2;
        // const N_CHANNELS : i32 = 4;
        // const pixels : [u8; (PIX_WIDTH * PIX_HEIGHT * N_CHANNELS) as usize] = [
        //     0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0xff,
        //     0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
        // ];

        unsafe {
            // --- CHAFA CONFIG --- //
        
            // performance note:
            // "The number of available symbols is a significant factor in the speed...
            // For the fastest possible operation you could use a single symbol --
            // CHAFA_SYMBOL_TAG_VHALF works well by itself."
            //
            // https://hpjansson.org/chafa/ref/chafa-ChafaSymbolMap.html

            let symbol_map = chafa_symbol_map_new();

            chafa_symbol_map_add_by_tags(symbol_map, config.symbols);
            // chafa_symbol_map_remove_by_tags(symbol_map, ChafaSymbolTags_CHAFA_SYMBOL_TAG_BRAILLE);

            let canvas_config = chafa_canvas_config_new();
            chafa_canvas_config_set_geometry(canvas_config, 
                                             config.cols as i32,
                                             config.rows as i32);
            chafa_canvas_config_set_symbol_map(canvas_config, symbol_map);
            chafa_canvas_config_set_work_factor(canvas_config, config.quality);

            let canvas = chafa_canvas_new(canvas_config);

            // --- OUTPUT --- //

            chafa_canvas_draw_all_pixels(canvas,
                                         ChafaPixelType_CHAFA_PIXEL_RGBA8_UNASSOCIATED,
                                         pixels.as_ptr(),
                                         src_width,
                                         src_height,
                                         src_width * num_channels); // rowstride

            // deperecated since chafa v1.6, even tho it's simpler
            let gstr : *mut _GString = chafa_canvas_build_ansi(canvas);

            // or this encouraged alternative to chafa_canvas_build_ansi(), 
            // although I haven't figured out the term_info part yet:
            // let term_info = chafa_term_info_new();
            // let gstr : *mut _GString = chafa_canvas_print(canvas, term_info); 


            // wrapping raw bytes of C strings into friendly Rust types,
            // so output can be manipulated in Rust and passed to println!
            //
            // https://doc.rust-lang.org/std/ffi/struct.CStr.html
            let cstr = std::ffi::CStr::from_ptr((*gstr).str_);
            let result = String::from_utf8_lossy(cstr.to_bytes()).to_string();
            // 
            // instead of using glib print helpers like
            // g_print((*result_gstr).str_);
            // g_printerr((*result_gstr).str_);

            // --- CLEANUP --- //

            g_string_free(gstr, 1);
            chafa_canvas_unref(canvas);
            chafa_canvas_config_unref(canvas_config);
            chafa_symbol_map_unref(symbol_map);
            // chafa_term_info_unref(term_info);
            return Ok(result);
        };
}

// chafa_canvas_draw_all_pixels
// fn chafa_canvas_draw_all_pixels (
//      canvas : * mut ChafaCanvas ,
//      src_pixel_type : ChafaPixelType ,
//      src_pixels : * const guint8 ,
//      src_width : gint ,
//      src_height : gint ,
//      src_rowstride : gint
//  )
// "Replaces pixel data of canvas with a copy of that found at src_pixels"
  
  
// struct ChafaImage { _unused : [u8 ; 0] , } 
// fn chafa_image_new () -> * mut ChafaImage ;
//
// struct ChafaFrame { _unused : [u8 ; 0] , } 
// fn chafa_frame_new (
//      data : gconstpointer ,
//      pixel_type : ChafaPixelType ,
//      width : gint ,
//      height : gint ,
//      rowstride : gint
// ) -> * mut ChafaFrame ;
  
  
// struct ChafaSymbolMap { _unused : [u8 ; 0] , }
// pub type ChafaSymbolTags = :: std :: os :: raw :: c_int
// const ChafaSymbolTags_CHAFA_SYMBOL_TAG_ALL : ChafaSymbolTags = - 1075314689
// fn chafa_symbol_map_add_by_tags (symbol_map : * mut ChafaSymbolMap , tags : ChafaSymbolTags) 
// chafa_symbol_map_add_by_range( ..., gunichar first, gunichar last)
// type gunichar = guint32
// https://doc.rust-lang.org/std/primitive.char.html
  
  
// struct ChafaTermInfo { _unused : [u8 ; 0] , } 
// fn chafa_term_info_new () -> * mut ChafaTermInfo ;
//
// > "A ChafaTermInfo describes the characteristics of one particular kind of display terminal. It
// > stores control sequences that can be used to move the cursor, change text attributes, mark the
// > beginning and end of sixel graphics data, etc.
// > ChafaTermInfo also implements an efficient low-level API for formatting these sequences with
// > marshaled arguments so they can be sent to the terminal."


// chafa uses three different types of dimensions:
// - src height and width (ie. image size)
// - destination height and width (eg. viewport size) (ie. max canvas dimensions)
// - canvas height and width (which could be padded around to fit the viewport)
//
// example:                      viewport
//                               300x200
//                              ┌──────┌──────────┐──────┐ 
//  src_image                   │      │  canvas  │      │ 
//    10x20                     │      │  100x200 │      │ 
//    ┌───┐                     │      │          │      │ 
//    │   │                     │      │          │      │ 
//    │   │                     │      │          │      │ 
//    └───┘                     │      │          │      │ 
//                              └──────└──────────┘──────┘ 
//
// Canvas holds Image, and Image holds Frame



// chafa_canvas_set_geometry()
// chafa_symbol_map_new()
// chafa_symbol_map_add_by_tags(symbol_map, ChafaSymbolTags_CHAFA_SYMBOL_TAG_ALL)
// chafa_canvas_config_set_symbol_map(config, symbol_map)
