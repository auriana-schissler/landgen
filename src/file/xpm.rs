use crate::color::Color;
use crate::get_commandline_footer;
use crate::render::RenderState;
use std::io;
use std::io::Write;
use std::sync::Arc;

fn get_chars_per_pixel(color_table_len: usize, chars: &[u8]) -> u8 {
    (color_table_len as f64).log(chars.len() as f64).ceil() as u8
}

#[test]
fn test_chars_per_pixel() {
    let chars = b"@$.,:;-+=#*&ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    assert_eq!(get_chars_per_pixel(63, chars), 1);
    assert_eq!(get_chars_per_pixel(255, chars), 2);
    assert_eq!(get_chars_per_pixel(5000, chars), 3);
    assert_eq!(get_chars_per_pixel(65535, chars), 3);
}

fn get_chars(chars: &[u8], color_index: usize, chars_per_pixel: u8) -> String {
    let mut color_index = color_index;
    let mut retval: String = String::with_capacity(chars_per_pixel as usize);
    for _ in 0..chars_per_pixel {
        retval.push(chars[color_index & 0b111111] as char);
        color_index /= chars.len();
    }
    retval
}

// TODO: Someday, add monochrome saving, perhaps
pub(super) fn write_to<W: Write>(state: Arc<RenderState>, writer: &mut W) -> Result<(), io::Error> {
    //Character table for XPM output
    let chars = b"@$.,:;-+=#*&ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let cmdline = get_commandline_footer();
    let chars_per_pixel = get_chars_per_pixel(state.color_table.len(), chars);

    writeln!(writer, "/* XPM */")?;
    writeln!(writer, "/* Command line: */")?;
    writeln!(writer, "/* {}*/", cmdline)?;
    writeln!(writer, "static char *xpmdata[] = {{")?;
    writeln!(writer, "/* width height ncolors chars_per_pixel */")?;
    writeln!(
        writer,
        "\"{} {} {} {}\",",
        state.options.width,
        state.options.height,
        state.color_table.len(),
        chars_per_pixel
    )?;

    writeln!(writer, "/* colors */")?;
    for i in 0..state.color_table.len() {
        let Color { red, green, blue } = &state.color_table[i];
        writeln!(writer, "\"{} c #{red:2x}{green:2x}{blue:2x}\",", get_chars(chars, i, chars_per_pixel))?;
    }

    writeln!(writer, "/* pixels */")?;
    for v in state.canvas.read().unwrap().iter() {
        for h in v.iter() {
            writeln!(writer, "\"")?;
            for color_index in h.iter().map(|x| *x as usize) {
                writeln!(writer, "{}", get_chars(chars, color_index, chars_per_pixel))?;
            }
            writeln!(writer, "\",")?;
        }
    }
    writeln!(writer, "}};")?;
    writer.flush()?;
    Ok(())
}