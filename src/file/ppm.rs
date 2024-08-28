use crate::get_commandline_footer;
use crate::render::RenderState;
use std::io;
use std::io::Write;
use std::sync::Arc;

pub(super) fn write_to<W: Write>(state: Arc<RenderState>, writer: &mut W) -> Result<(), io::Error> {
    writeln!(writer, "P6")?;
    writeln!(writer, "#fractal planet image")?;
    writeln!(writer, "# Command line:")?;
    writeln!(writer, "# {}", get_commandline_footer())?;
    writeln!(
        writer,
        "{} {} 255",
        state.options.width, state.options.height
    )?;

    for (vi, v) in state.canvas.read().unwrap().iter().enumerate() {
        for (hi, h) in v.iter().rev().enumerate() {
            for (i, color_index) in h.iter().enumerate().map(|x| (x.0, *x.1 as usize)) {
                let color = &state.color_table[color_index];

                if state.options.shading_level > 0 {
                    let shade = state.shading[vi][hi][i] as u32;
                    writer.write_all(&[
                        ((shade * color.blue as u32) / 150).min(255) as u8,
                        ((shade * color.green as u32) / 150).min(255) as u8,
                        ((shade * color.red as u32) / 150).min(255) as u8,
                    ])?;
                } else {
                    writer.write_all(&[color.blue, color.green, color.red])?;
                }
            }
        }
    }

    writer.flush()?;
    Ok(())
}
