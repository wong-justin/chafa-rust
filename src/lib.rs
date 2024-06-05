mod chafa;

use chafa_sys::*;

use image::ImageBuffer;

// --- hacky bitflags --- //

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

pub mod PixelType {
    // std::os::raw:c_uint is pretty much u32
    pub const RGBA8_PREMULTIPLIED : u32 = chafa_sys::ChafaPixelType_CHAFA_PIXEL_RGBA8_PREMULTIPLIED;
    pub const BGRA8_PREMULTIPLIED : u32 = chafa_sys::ChafaPixelType_CHAFA_PIXEL_BGRA8_PREMULTIPLIED;
    pub const ARGB8_PREMULTIPLIED : u32 = chafa_sys::ChafaPixelType_CHAFA_PIXEL_ARGB8_PREMULTIPLIED;
    pub const ABGR8_PREMULTIPLIED : u32 = chafa_sys::ChafaPixelType_CHAFA_PIXEL_ABGR8_PREMULTIPLIED;
    pub const RGBA8_UNASSOCIATED : u32 = chafa_sys::ChafaPixelType_CHAFA_PIXEL_RGBA8_UNASSOCIATED;
    pub const BGRA8_UNASSOCIATED : u32 = chafa_sys::ChafaPixelType_CHAFA_PIXEL_BGRA8_UNASSOCIATED;
    pub const ARGB8_UNASSOCIATED : u32 = chafa_sys::ChafaPixelType_CHAFA_PIXEL_ARGB8_UNASSOCIATED;
    pub const ABGR8_UNASSOCIATED : u32 = chafa_sys::ChafaPixelType_CHAFA_PIXEL_ABGR8_UNASSOCIATED;
    pub const RGB8 : u32 = chafa_sys::ChafaPixelType_CHAFA_PIXEL_RGB8;
    pub const BGR8 : u32 = chafa_sys::ChafaPixelType_CHAFA_PIXEL_BGR8;
}

// --- structs holding C pointers and associated functions --- //
// most of the _ptrs are for naive structs that look like { _unused : [u8 ; 0] , }

pub struct SymbolMap {
    _ptr: *mut ChafaSymbolMap,
}

pub type SymbolTagsFlag = i32;
pub type PixelTypeFlag = u32;

impl SymbolMap {

    pub fn new() -> Self {
        unsafe {
            Self {
                _ptr : chafa_sys::chafa_symbol_map_new()
            }
        }
    }

    // performance note:
    // "The number of available symbols is a significant factor in the speed...
    // For the fastest possible operation you could use a single symbol --
    // CHAFA_SYMBOL_TAG_VHALF works well by itself."
    //
    // https://hpjansson.org/chafa/ref/chafa-ChafaSymbolMap.html
    pub fn add_by_tags(&self, symbol_tags : SymbolTagsFlag) {
        unsafe {
            chafa_symbol_map_add_by_tags(self._ptr, symbol_tags);
        }
    }

    // TODO:
    // remove_by_tags
    // add_by_range
    // remove_by_range

    // chafa_symbol_map_add_by_range( ..., gunichar first, gunichar last)
    // type gunichar = guint32
    // https://doc.rust-lang.org/std/primitive.char.html
}

impl core::ops::Drop for SymbolMap {
    fn drop(&mut self) {
        unsafe {
            chafa_symbol_map_unref(self._ptr);
        }
    }
}

pub struct Config {
    _ptr: *mut ChafaCanvasConfig,
}

impl Config {
    pub fn new() -> Self {
        unsafe {
            Self {
                _ptr : chafa_sys::chafa_canvas_config_new()
            }
        }
    }

    pub fn set_geometry(&self, width : i32, height : i32) {
        unsafe {
            chafa_sys::chafa_canvas_config_set_geometry(self._ptr, 
                                                        width,
                                                        height);
        }
    }

    pub fn set_symbol_map(&self, symbol_map : SymbolMap) {
        unsafe {
            chafa_sys::chafa_canvas_config_set_symbol_map(self._ptr,
                                                          symbol_map._ptr);
        }
    }

    pub fn set_work_factor(&self, work_factor : f32) {
        // work_factor from 0.0 to 1.0
        unsafe {
            chafa_sys::chafa_canvas_config_set_work_factor(self._ptr,
                                                           work_factor);
        }
    }

    // TODO:
    // all the other config options
}

impl core::ops::Drop for Config {
    fn drop(&mut self) {
        unsafe {
            chafa_canvas_config_unref(self._ptr);
        }
    }
}

pub struct Canvas {
    _ptr: *mut ChafaCanvas,
}

impl Canvas {
    pub fn new(config : Config) -> Self {
        unsafe {
            Self {
                _ptr : chafa_sys::chafa_canvas_new(config._ptr)
            }
        }
    }

    // "Replaces pixel data of canvas with a copy of that found at src_pixels"
    pub fn draw_all_pixels(&self,
                                  pixel_type : PixelTypeFlag,
                                  pixels : &[u8],
                                  width: i32,    
                                  height: i32,   
                                  rowstride: i32)
    {
        unsafe {
         chafa_canvas_draw_all_pixels(self._ptr,
                                      pixel_type,
                                      pixels.as_ptr(),
                                      width,
                                      height,
                                      rowstride);
        }
    }

    pub fn build_ansi(&self) -> String {
        unsafe {
            let gstr : *mut _GString = chafa_canvas_build_ansi(self._ptr);

            // wrapping raw bytes of C strings into friendly Rust types,
            // so output can be manipulated in Rust and passed to println,
            // instead of using glib print helpers like
            // g_print((*result_gstr).str_); or
            // g_printerr((*result_gstr).str_);
            //
            // https://doc.rust-lang.org/std/ffi/struct.CStr.html
            let cstr = std::ffi::CStr::from_ptr((*gstr).str_);
            let result = String::from_utf8_lossy(cstr.to_bytes()).to_string();

            g_string_free(gstr, 1);
            return result;
        }
    }

    // TODO:
    // print(&self, term_info)
    // and other functions
}

impl core::ops::Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            chafa_canvas_unref(self._ptr);
        }
    }
}


// --- convience abstracted functions --- //


// TODO potential functions:
//
// fn image2pixels - when you need image data but don't want to display yet
// fn pixels2ansi - when frames come from other sources, like reading video frames
// fn images2ansi - keep the canvas around for continuous use, eg. animated gifs


// very similar to: https://hpjansson.org/chafa/ref/chafa-using.html
pub fn image2ansi<P>(path: P, (cols, rows) : (u32, u32)) -> Result<String, image::ImageError>
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
    let src_width : i32 = img.width().try_into().unwrap();
    let src_height : i32 = img.height().try_into().unwrap();
    let pixels : Vec<u8> = img.into_raw();

    // --- CHAFA CONFIG --- //
    

    let symbol_map = SymbolMap::new();
    symbol_map.add_by_tags(Symbols::BLOCK);

    let config = Config::new();
    config.set_geometry(cols as i32, rows as i32);
    config.set_symbol_map(symbol_map);
    config.set_work_factor(1.0);

    let canvas = Canvas::new(config);

    // --- OUTPUT --- //
    
    canvas.draw_all_pixels(PixelType::RGBA8_UNASSOCIATED,
                           &pixels,
                           src_width,
                           src_height,
                           src_width * num_channels);


    let result = canvas.build_ansi();
    return Ok(result);
}

  
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_pixel_output() {
        // just a red block char
        const PIX_WIDTH : i32 = 1;
        const PIX_HEIGHT : i32 = 1;
        const N_CHANNELS : i32 = 4;
        let pixels : [u8; 4] = [0xff, 0x00, 0x00, 0xff];

        let symbol_map = SymbolMap::new();
        symbol_map.add_by_tags(Symbols::BLOCK);

        let config = Config::new();
        config.set_geometry(1, 1);
        config.set_symbol_map(symbol_map);

        let canvas = Canvas::new(config);

        canvas.draw_all_pixels(PixelType::RGBA8_UNASSOCIATED,
                               &pixels,
                               PIX_WIDTH,
                               PIX_HEIGHT,
                               PIX_WIDTH * N_CHANNELS);

        let output : String = canvas.build_ansi();
        assert_eq!(output, "[0m[38;2;254;0;0m█[0m");
    }
}
