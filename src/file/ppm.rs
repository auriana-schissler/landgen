use crate::get_commandline_footer;
use crate::render::RenderState;
use std::io;
use std::io::Write;
use std::sync::Arc;

pub(super) fn write_to<W: Write>(state: Arc<RenderState>, writer: &mut W) -> Result<(), io::Error> {
    writeln!(writer, "P6\n{}\n{}\n255",
             state.options.slicing.width as u32, state.options.slicing.height as u32)?;

    let shading = state.shading.read().unwrap();
    
    for (vi, v) in state.canvas.read().unwrap().iter().enumerate() {
        for (hi, h) in v.iter().enumerate() {
            for (wi, color_index) in h.iter().enumerate().map(|x| (x.0, *x.1 as usize)) {
                let color = &state.options.color_table[color_index];

                if state.options.shading_level > 0 {
                    let shade = shading[vi][hi][wi] as u32;
                    writer.write_all(&[
                        ((shade * color.red as u32) / 150).min(255) as u8,
                        ((shade * color.green as u32) / 150).min(255) as u8,
                        ((shade * color.blue as u32) / 150).min(255) as u8,
                    ])?;
                } else {
                    writer.write_all(&[color.red, color.green, color.blue])?;
                }
            }
        }
    }
    writeln!(writer, "#fractal planet image")?;
    writeln!(writer, "#{}", get_commandline_footer())?;

    writer.flush()?;
    Ok(())
}
