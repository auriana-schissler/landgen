pub mod bitmap;
mod heightfield;
pub mod png;
pub mod ppm;
pub mod xpm;

use crate::render::RenderState;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::sync::Arc;

#[allow(non_camel_case_types, non_upper_case_globals)]
#[derive(Clone)]
pub enum FileType {
    bmp = 1,
    ppm = 2,
    xpm = 3,
    heightfield = 4,
    png = 5,
}

pub enum ColorMode {
    Color,
    Monochrome,
}

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
    if state.options.output_file.is_some() {
        let file = File::create(state.options.output_file.to_owned().unwrap())?;
        write_to(state.clone(), &mut BufWriter::new(file))
    } else {
        write_to(state.clone(), &mut BufWriter::new(io::stdout()))
    }
}

fn write_to<W: Write>(state: Arc<RenderState>, writer: &mut W) -> Result<(), io::Error> {
    match state.options.filetype {
        FileType::bmp => bitmap::write_to(state.clone(), writer),
        FileType::heightfield => heightfield::write_to(state.clone(), writer),
        FileType::ppm => ppm::write_to(state.clone(), writer),
        FileType::xpm => xpm::write_to(state.clone(), writer),
        FileType::png => png::write_to(state.clone(), writer),
    }
}
