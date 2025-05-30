pub mod bitmap;
mod heightfield;
mod png;
mod ppm;
mod svg;
mod xpm;

use crate::render::RenderState;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::sync::Arc;

#[allow(non_camel_case_types, non_upper_case_globals)]
#[derive(Clone, PartialEq)]
pub enum FileType {
    bmp,
    ppm,
    xpm,
    heightfield,
    png,
}

pub enum ColorMode {
    Color,
    Monochrome,
}

pub fn get_file_extension<'a>(filetype: &FileType) -> &'a str {
    match filetype {
        FileType::bmp => ".bmp",
        FileType::ppm => ".ppm",
        FileType::xpm => ".xpm",
        FileType::heightfield => ".heightfield",
        FileType::png => ".png",
    }
}

pub fn write_to_file(state: Arc<RenderState>) -> Result<(), io::Error> {
    let filename = state.options.output_file.clone().unwrap();
    for filetype in &state.options.filetypes {
        let mut filename = filename.to_owned();
        filename.push_str(get_file_extension(filetype));
        let file = File::create(&filename)?;
        write_to(state.clone(), filetype, &mut BufWriter::new(file))?;
    }
    Ok(())
}

pub fn write_to_stdout(state: Arc<RenderState>) -> Result<(), io::Error> {
    write_to(
        state.clone(),
        &state.options.filetypes[0],
        &mut BufWriter::new(io::stdout()),
    )
}

pub fn write_out<W: Write>(state: Arc<RenderState>, writer: &mut W) -> Result<(), io::Error> {
    for filetype in &state.options.filetypes {
        write_to(state.clone(), filetype, writer)?;
    }
    Ok(())
}

pub fn write_out_raw<W: Write>(state: Arc<RenderState>, writer: &mut W) -> Result<(), io::Error> {
    for filetype in &state.options.filetypes {
        write_to(state.clone(), filetype, writer)?;
    }
    Ok(())
}

// pub fn write_file(state: Arc<RenderState>) -> Result<(), io::Error> {
//     if let Some(filename) = state.options.output_file.clone() {
//         for filetype in &state.options.filetypes {
//             let mut filename = filename.to_owned();
//             filename.push_str(get_file_extension(filetype));
//             let file = File::create(&filename)?;
//             write_to(state.clone(), filetype, &mut BufWriter::new(file))?;
//         }
//         Ok(())
//     } else {
//         write_to(
//             state.clone(),
//             &state.options.filetypes[0],
//             &mut BufWriter::new(io::stdout()),
//         )
//     }
// }

fn write_to<W: Write>(
    state: Arc<RenderState>,
    filetype: &FileType,
    writer: &mut W,
) -> Result<(), io::Error> {
    match filetype {
        FileType::bmp => bitmap::write_to(state.clone(), writer),
        FileType::heightfield => heightfield::write_to(state.clone(), writer),
        FileType::ppm => ppm::write_to(state.clone(), writer),
        FileType::xpm => xpm::write_to(state.clone(), writer),
        FileType::png => png::write_to(state.clone(), writer),
    }
}
