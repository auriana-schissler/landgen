use crate::get_commandline_footer;
use crate::render::RenderState;
use mtpng::encoder::{Encoder, Options};
use mtpng::{ColorType, CompressionLevel, Header};
use std::io;
use std::io::Write;
use std::sync::Arc;

// TODO: Add indexed palette if no shading exists, and embed commandline
pub(super) fn write_to<W: Write>(state: Arc<RenderState>, writer: &mut W) -> Result<(), io::Error> {
    // PNG file specification
    // https://en.wikipedia.org/wiki/PNG

    let _cmdline = get_commandline_footer();

    let mut header = Header::new();
    header.set_size(state.options.width as u32, state.options.height as u32)?;
    header.set_color(ColorType::Truecolor, 8)?;

    let mut options = Options::new();
    options.set_compression_level(CompressionLevel::Default)?;
    options.set_streaming(true)?;

    let mut encoder = Encoder::new(writer, &options);
    encoder.write_header(&header)?;
    
    // TODO: detect if any color-altering options are enabled, and switch to writing a palette instead 

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