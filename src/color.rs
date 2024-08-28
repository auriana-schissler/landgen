use crate::render::RenderOptions;
use crate::util::unwrap_or_return;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct ColorTable {
    rows: Vec<Color>,
    pub black: u16,
    pub white: u16,
    pub back: u16,
    pub grid: u16,
    pub outline1: u16,
    pub outline2: u16,
    pub sea_bottom: u16,
    pub sea_level: u16,
    pub lowest_land: u16,
    pub highest_land: u16,
    pub sea_depth: u16,
    pub land_height: u16,
}

impl ColorTable {
    pub fn new(size: usize) -> Self {
        Self {
            rows: vec![Color::new(); size],
            black: 0,
            white: 1,
            back: 2,
            grid: 3,
            outline1: 4,
            outline2: 5,
            sea_bottom: 6,
            sea_level: 7,
            lowest_land: 8,
            highest_land: 9,
            sea_depth: 0,
            land_height: 0
        }
    }

    /// Returns true if the table contains only black and white
    ///
    /// Reasonably this _could_ be written to be true if it's two distinct colors
    /// But that would be a feature upgrade
    pub fn is_monochrome(&self) -> bool {
        for row in &self.rows {
            match row {
                Color {
                    red: 0,
                    green: 0,
                    blue: 0,
                } => {}
                Color {
                    red: 255,
                    green: 255,
                    blue: 255,
                } => {}
                _ => {
                    return false;
                }
            }
        }
        true
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

impl Index<usize> for ColorTable {
    type Output = Color;

    fn index(&self, i: usize) -> &Self::Output {
        &self.rows[i]
    }
}

impl IndexMut<usize> for ColorTable {
    fn index_mut(&mut self, i: usize) -> &mut Color {
        &mut self.rows[i]
    }
}

#[derive(Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new() -> Self {
        Self::from_colors(0, 0, 0)
    }

    pub fn from_colors(r: u8, g: u8, b: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ColorRow {
    pub index: usize,
    pub color: Color,
}

impl ColorRow {
    pub fn from_index_and_colors(index: usize, r: u8, g: u8, b: u8) -> Self {
        Self {
            index,
            color: Color::from_colors(r, g, b),
        }
    }
}

const T: usize = 'T' as usize - 64;
const G: usize = 'G' as usize - 64;
const B: usize = 'B' as usize - 64;
const D: usize = 'D' as usize - 64;
const S: usize = 'S' as usize - 64;
const F: usize = 'F' as usize - 64;
const R: usize = 'R' as usize - 64;
const W: usize = 'W' as usize - 64;
const E: usize = 'E' as usize - 64;
const O: usize = 'O' as usize - 64;
const I: usize = 'I' as usize - 64;

// TODO: use include_str!() to embed color data into the program
pub fn build_color_data(options: &RenderOptions) -> ColorTable {
    let mut table = generate_color_data(&options.color_filename);

    if options.show_biomes {
        let lowest_land = table.lowest_land as usize;
        table[T + lowest_land] = Color::from_colors(210, 210, 210);
        table[G + lowest_land] = Color::from_colors(250, 215, 165);
        table[B + lowest_land] = Color::from_colors(105, 155, 120);
        table[D + lowest_land] = Color::from_colors(220, 195, 175);
        table[S + lowest_land] = Color::from_colors(225, 155, 100);
        table[F + lowest_land] = Color::from_colors(155, 215, 170);
        table[R + lowest_land] = Color::from_colors(170, 195, 200);
        table[W + lowest_land] = Color::from_colors(185, 150, 160);
        table[E + lowest_land] = Color::from_colors(130, 190, 25);
        table[O + lowest_land] = Color::from_colors(110, 160, 170);
        table[I + lowest_land] = Color::from_colors(255, 255, 255);
    }

    table
}

fn get_color_steps(
    start_index: usize,
    sc: &Color,
    end_index: usize,
    ec: &Color,
) -> (f64, f64, f64) {
    let index_diff = end_index - start_index;
    (
        (ec.red as i32 - sc.red as i32) as f64 / index_diff as f64,
        (ec.green as i32 - sc.green as i32) as f64 / index_diff as f64,
        (ec.blue as i32 - sc.blue as i32) as f64 / index_diff as f64,
    )
}

#[test]
fn test_color_step_calc() {
    let start = Color::from_colors(0, 0, 0);
    let end = Color::from_colors(255, 255, 255);
    let (red, green, blue) = get_color_steps(0, &start, 255, &end);

    assert_eq!(red, 1.);
    assert_eq!(green, 1.);
    assert_eq!(blue, 1.);
}

#[test]
fn test_negative_color_step_calc() {
    let start = Color::from_colors(238, 170, 34);
    let end = Color::from_colors(221, 136, 34);
    let (red, green, blue) = get_color_steps(49, &start, 51, &end);

    assert_eq!(red, -8.5);
    assert_eq!(green, -17.);
    assert_eq!(blue, 0.);
}

// Format of colour file is a sequence of lines
// each consisting of four integers:
// colour_number red green blue
// where 0 <= colour_number <= 65535
// and 0 <= red, green, blue <= 255
// The colour numbers must be increasing
// The first colours have special uses:
// 0 is usually black (0,0,0)
// 1 is usually white (255,255,255)
// 2 is the background colour
// 3 is used for latitude/longitude grid lines
// 4 and 5 are used for outlines and contour lines
// 6 upwards are used for altitudes
// Halfway between 6 and the max colour is sea level
// Shallowest sea is (max+6)/2 and land is above this
// With 65536 colours, (max+6)/2 = 32770
// Colours between specified are interpolated

/// Reads color rows from the specified file and interpolates color data where needed
fn generate_color_data(filename: &str) -> ColorTable {
    let color_rows = read_color_file(filename);
    validate_color_file(&color_rows);
    let max_index = color_rows.iter().map(|x| x.index).max().unwrap_or(0);
    let mut table = ColorTable::new(max_index + 1);
    let mut last_good_index = 0;

    for row in color_rows {
        table[row.index] = row.color;

        let index_diff = row.index - last_good_index;
        if index_diff > 1 {
            let start_color = table[last_good_index].clone();
            let (red_step, green_step, blue_step) = get_color_steps(
                last_good_index,
                &table[last_good_index],
                row.index,
                &table[row.index],
            );

            for d in 1..index_diff {
                table[last_good_index + d] = Color::from_colors(
                    (start_color.red as i16 + (red_step * d as f64) as i16) as u8,
                    (start_color.green as i16 + (green_step * d as f64) as i16) as u8,
                    (start_color.blue as i16 + (blue_step * d as f64) as i16) as u8,
                );
            }
        }
        last_good_index = row.index;
    }

    let low_point = 6;
    let highest_land = last_good_index;
    let sea_level = (low_point + highest_land) / 2;
    let lowest_land = sea_level + 1;

    table.highest_land = highest_land as u16;
    table.sea_level = sea_level as u16;
    table.lowest_land = lowest_land as u16;

    table.sea_depth = table.sea_level - table.sea_bottom;
    table.land_height = table.highest_land - table.lowest_land;
    table
}

fn read_color_file(filename: &str) -> Vec<ColorRow> {
    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            reader
                .lines()
                .map(|x| x.unwrap_or("".into()))
                .filter(|x| !x.trim().is_empty())
                .map(|x| {
                    get_color_line_values(x).unwrap_or_else(|e| {
                        eprintln!("Error parsing color file with error {:?}", e);
                        panic!()
                    })
                })
                .collect::<Vec<ColorRow>>()
        }
        Err(e) => {
            eprintln!("Error reading color file! {:?}", e);
            panic!()
        }
    }
}

fn validate_color_file(rows: &[ColorRow]) {
    for (i, row) in rows[0..=6].iter().enumerate() {
        if row.index != i {
            eprintln!("Color file is missing a portion of the first 7 entries. Please ensure items 0 through 6 exist.");
            panic!()
        }
    }
}

#[test]
fn test_read_color_file() {
    use std::env;

    let mut filepath = env::var("CARGO_MANIFEST_DIR").unwrap();
    filepath.push_str("\\src\\color_files\\landmask.col");
    let colors = read_color_file(&filepath);

    assert_eq!(colors.last().unwrap().index, 9);
    assert_eq!(colors[0].color.red, 0); // verifying that values
    assert_eq!(colors[1].color.red, 255); // are all correct and
}

#[derive(Debug)]
enum ColorFileParseError {
    IndexParse,
    ColorParse,
    TooFewTokens,
}

fn get_color_line_values(line: String) -> Result<ColorRow, ColorFileParseError> {
    let tokens: Vec<&str> = line.split_ascii_whitespace().collect();

    if tokens.len() > 3 {
        Ok(ColorRow::from_index_and_colors(
            unwrap_or_return!(
                tokens[0].parse::<usize>(),
                Err(ColorFileParseError::IndexParse)
            )
            .min(u16::MAX as usize),
            unwrap_or_return!(tokens[1].parse(), Err(ColorFileParseError::ColorParse)),
            unwrap_or_return!(tokens[2].parse(), Err(ColorFileParseError::ColorParse)),
            unwrap_or_return!(tokens[3].parse(), Err(ColorFileParseError::ColorParse)),
        ))
    } else {
        Err(ColorFileParseError::TooFewTokens)
    }
}

#[test]
fn test_read_color_line() {
    let row = get_color_line_values("      5    9     22 99".to_string()).unwrap();

    assert_eq!(row.index, 5);
    assert_eq!(row.color.red, 9); // verifying that values
    assert_eq!(row.color.green, 22); // are all correct and
    assert_eq!(row.color.blue, 99); // final value is extended onward
}

#[test]
#[should_panic]
fn test_read_bad_color_line() {
    get_color_line_values("      5    -     22 999".to_string()).unwrap();
}

#[test]
fn test_color_file_interpolation() {
    use std::env;

    let mut filepath = env::var("CARGO_MANIFEST_DIR").unwrap();
    filepath.push_str("\\src\\color_files\\greyscale.col");
    let table = generate_color_data(&filepath);

    assert_eq!(table.highest_land, 261);
    assert_eq!(table.len(), 262);

    for i in 6..=261 {
        assert_eq!(table[i].red, (i - 6) as u8);
    }
}

#[test]
fn test_olsson_color_file_interpolation() {
    use std::env;

    let mut filepath = env::var("CARGO_MANIFEST_DIR").unwrap();
    filepath.push_str("\\src\\color_files\\olsson.col");
    let table = generate_color_data(&filepath);

    assert_eq!(table.highest_land, 66);
    assert_eq!(table.len(), 67);

    assert_eq!(table[49].red, 238);
    assert_eq!(table[49].green, 170);
    assert_eq!(table[49].blue, 34);

    assert_eq!(table[50].red, 230);
    assert_eq!(table[50].green, 153);
    assert_eq!(table[50].blue, 34);

    assert_eq!(table[51].red, 221);
    assert_eq!(table[51].green, 136);
    assert_eq!(table[51].blue, 34);
}
