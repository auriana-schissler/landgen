use crate::file::ColorMode;
use crate::get_commandline_footer;
use crate::render::RenderState;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use mtpng::{ColorType, CompressionLevel, Header};
use mtpng::encoder::{Encoder, Options};

pub(super) fn write(state: Arc<RenderState>) -> Result<(), io::Error> {
    if state.options.output_file.is_some() {
        let file = File::create(state.options.output_file.to_owned().unwrap())?;
        let mut writer = BufWriter::new(file);
        write_to(state.clone(), &mut writer)
    } else {
        let mut writer = BufWriter::new(io::stdout());
        write_to(state.clone(), &mut writer)
    }
}

fn write_to<W: Write>(state: Arc<RenderState>, writer: &mut W) -> Result<(), io::Error> {
    // PNG file specification
    // https://en.wikipedia.org/wiki/PNG

    let cmdline = get_commandline_footer();

    let mut header = Header::new();
    header.set_size(state.options.width as u32, state.options.height as u32)?;
    header.set_color(ColorType::Truecolor, 8)?;

    let mut options = Options::new();
    options.set_compression_level(CompressionLevel::High)?;
    options.set_streaming(false)?;

    let mut encoder = Encoder::new(writer, &options);

    encoder.write_header(&header)?;

    let canvas = state.canvas.read().unwrap();
    
    let height = state.options.height as usize;
    let width = state.options.width as usize;
    let mut line: Vec<u8> = Vec::with_capacity(height * width * 3);
    for v in canvas.iter() {
        for h in v.iter() {
            for w in h.iter() {
                let color_index = *w as usize;
                let color = &state.color_table[color_index];
                line.push(color.red);
                line.push(color.green);
                line.push(color.blue);
            }
        }
    }
    encoder.write_image_rows(&line)?;
    //encoder.write_chunk(b"CMDL", cmdline.as_bytes())?;
    encoder.finish()?;
    Ok(())
}

// (bits per pixel, indexed colors, color type)
fn get_image_info(state: &RenderState) -> (u8, u8, u8) {
    match state.get_color_mode() {
        ColorMode::Color => (8, 0, 2),
        ColorMode::Monochrome => (1, 2, 0),
    }
}
