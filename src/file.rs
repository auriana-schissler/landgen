pub mod bitmap;
pub mod png;
pub mod ppm;
pub mod xpm;
mod heightfield;

use std::io;
use std::sync::Arc;
use crate::render::RenderState;


#[allow(non_camel_case_types, non_upper_case_globals)]
#[derive(Clone)]
pub enum FileType {
    bmp = 1,
    ppm = 2,
    xpm = 3,
    heightfield = 4,
    png = 5,
}

pub enum ColorMode { Color, Monochrome }

pub fn get_file_extension<'a>(filetype: FileType) -> &'a str {
    match filetype {
        FileType::bmp => ".bmp",
        FileType::ppm => ".ppm",
        FileType::xpm => ".xpm",
        FileType::heightfield => ".heightfield",
        FileType::png => ".png",
    }
}

pub fn write_file(state: Arc<RenderState>) -> Result<(), io::Error> {
    match state.options.filetype {
        FileType::bmp => bitmap::write(state.clone()),
        FileType::heightfield => heightfield::write(state.clone()),
        FileType::ppm => ppm::write(state.clone()),
        FileType::xpm => xpm::write(state.clone()),
        FileType::png => png::write(state.clone()),
    }
}
