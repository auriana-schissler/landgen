use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use crate::file::ColorMode;
use crate::get_commandline_footer;
use crate::render::RenderState;

fn get_file_size(state: Arc<RenderState>) -> u64 {
    let (bpp, _, pixel_data_start) = get_bitmap_info(state.clone());
    let padded_width = ((state.options.width + 31) & 0b_1111_1111_1110_0000) as u64;
    get_commandline_footer().len() as u64
        + pixel_data_start as u64
        + (padded_width * state.options.height as u64 * bpp as u64) / 8
}

pub fn validate_size(state: Arc<RenderState>) -> bool {
    get_file_size(state) < u32::MAX as u64
}

// Returns a width padded to 4 byte boundaries
fn get_padded_width(state: Arc<RenderState>) -> u32 {
    match state.get_color_mode() {
        ColorMode::Color => (state.options.width as u32 + 3) & 0b_1111_1111_1111_1100,
        ColorMode::Monochrome => (state.options.width as u32 + 31) & 0b_1111_1111_1110_0000,
    }
}

pub(super) fn write(state: Arc<RenderState>) -> Result<(), io::Error> {
    if state.options.output_file.is_some() {
        let file = File::create(state.options.output_file.to_owned().unwrap())?;
        let mut writer = BufWriter::new(file);
        write_to(state, &mut writer)
    } else {
        let mut writer = BufWriter::new(io::stdout());
        write_to(state, &mut writer)
    }
}

fn write_to<W: Write>(state: Arc<RenderState>, writer: &mut W) -> Result<(), io::Error> {
    // Bitmap file specification
    // https://upload.wikimedia.org/wikipedia/commons/7/75/BMPfileFormat.svg
    // Bitmaps are considered malformed if height or width are over 32,768
    // A bitmap maximum size is 4GB and this program will error out if the expected size exceeds that

    let cmdline = get_commandline_footer();

    write!(writer, "BM")?;

    // bitmap row width is padded to a multiple of 4 bytes
    let padded_width = get_padded_width(state.clone());
    let color_mode = state.get_color_mode();
    let (bpp, colors, pixel_data_start) = get_bitmap_info(state.clone());
    let filesize = cmdline.len() as u32
        + pixel_data_start as u32
        + padded_width * state.options.height as u32 / 8;

    writer.write_all(&filesize.to_le_bytes())?;
    writer.write_all(&[
        0u8,
        0,
        0,
        0, // reserved space
        pixel_data_start,
        0,
        0,
        0, // index of pixel data
        40,
        0,
        0,
        0, // info header size
    ])?;
    writer.write_all(&(state.options.width as u32).to_le_bytes())?;
    writer.write_all(&(state.options.height as u32).to_le_bytes())?;
    writer.write_all(&[
        1u8, 0, // number of planes (1)
        bpp, 0, // bits per pixel
        0, 0, 0, 0, // compression level: none
        0, 0, 0, 0, // image size (unspecified)
        0, 32, 0, 0, // h. pixels/m
        0, 32, 0, 0, // v. pixels/m
        colors, 0, 0, 0, // colors in color table
        colors, 0, 0, 0, // important color count (0 = all)
    ])?;

    // writing our indexed colors
    if let ColorMode::Monochrome = color_mode {
        writer.write_all(&[
            0, 0, 0, 0, // black
            255, 255, 255, 255, // white
        ])?;
    };
panic!();
    // write pixels
    // let canvas = state.canvas.read().unwrap();
    // match color_mode {
    //     ColorMode::Color => {
    //         if state.options.shading_level > 0 {
    //             let shading = state.shading.read().unwrap();
    //             for h in (0..state.options.height).rev() {
    //                 for w in 0..state.options.width {
    //                     let shade = shading[w as usize][h as usize] as u32;
    //                     let color_index = canvas[w as usize][h as usize] as usize;
    //                     let color = &state.color_table[color_index];
    //                     writer.write_all(&[
    //                         ((shade * color.blue as u32) / 150).min(255) as u8,
    //                         ((shade * color.green as u32) / 150).min(255) as u8,
    //                         ((shade * color.red as u32) / 150).min(255) as u8,
    //                     ])?;
    //                 }
    //                 for _ in state.options.width as u32..padded_width {
    //                     writer.write_all(&[0])?;
    //                 }
    //             }
    //         } else {
    //             for h in (0..state.options.height).rev() {
    //                 for w in 0..state.options.width {
    //                     let color_index = canvas[w as usize][h as usize] as usize;
    //                     let color = &state.color_table[color_index];
    //                     writer.write_all(&[color.blue, color.green, color.red])?;
    //                 }
    //                 for _ in state.options.width as u32..padded_width {
    //                     writer.write_all(&[0])?;
    //                 }
    //             }
    //         }
    //     }
    //     ColorMode::Monochrome => {
    //         // we fit 32 pixels per 4 byte cluster
    //         for h in (0..state.options.height).rev() {
    //             for w in (0..padded_width).step_by(32) {
    //                 let mut quad = 0u32;
    //                 let stop = (state.options.width as u32 - w).min(32);
    // 
    //                 for s in 0..stop {
    //                     let color_index = canvas[(w + s) as usize][h as usize] as usize;
    //                     if (w + s) < state.options.width as u32
    //                         && state.color_table[color_index].red != 0
    //                     {
    //                         quad |= 0b1 << (31 - s);
    //                     }
    //                 }
    //                 writer.write_all(&quad.to_le_bytes())?;
    //             }
    //         }
    //     }
    // }

    write!(writer, "{}", cmdline)?;
    writer.flush()?;
    Ok(())
}

// (bits per pixel, indexed colors, pixel data index)
fn get_bitmap_info(state: Arc<RenderState>) -> (u8, u8, u8) {
    match state.get_color_mode() {
        ColorMode::Color => (24u8, 0u8, 54u8),
        ColorMode::Monochrome => (1, 2, 62),
    }
}
