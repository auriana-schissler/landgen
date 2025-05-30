use crate::color::Color;
use crate::render::RenderOptions;
use crate::util::Vec2D;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::sync::{RwLock, RwLockReadGuard};

pub struct VectorCanvas {
    background_color_index: u16,
    pub polygons: Vec<VectorPolygon>,
}

pub struct VectorPolygon {
    pub color_index: u16,
    pub origin: Point,
    pub deltas: Vec<Point>,
}

pub struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum ActiveSide {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq)]
enum ColorDirection {
    Sea,
    Land,
}

// WIP
pub fn vectorize(options: RenderOptions, canvas: RwLock<Vec<Vec2D<u16>>>) -> VectorCanvas {
    let mut discovered_borders = HashMap::<u16, Vec<Point>>::new();

    for color_index in options.color_table.sea_bottom..=options.color_table.land_peak {
        if color_index != options.color_table.sea_level {
            discovered_borders.insert(color_index, vec![]);
        }
    }
    let color_steps: u16 = 3; // this will come from options and control how many steps are taken till the next color
    let mut valid_color_indices = HashSet::<u16>::new();
    let mut current_step = 0;
    let mut last_color = Color::new();
    for color_index in options.color_table.sea_level - 1..=options.color_table.sea_bottom {
        let color = &options.color_table[color_index as usize];
        if last_color != *color {
            current_step += 1;
            last_color = color.clone();
            if current_step == color_steps {
                valid_color_indices.insert(color_index);
                current_step = 0;
            }
        }
    }
    valid_color_indices.insert(options.color_table.coastline);
    current_step = 0;
    last_color = Color::new();
    for color_index in options.color_table.coastline..=options.color_table.land_peak {
        let color = &options.color_table[color_index as usize];
        if last_color != *color {
            current_step += 1;
            last_color = color.clone();
            if current_step == color_steps {
                valid_color_indices.insert(color_index);
                current_step = 0;
            }
        }
    }

    let canvas_reader = canvas
        .read()
        .expect("Cannot open canvas for reading somehow.");

    for base_h in 0..options.slicing.height {
        for w in 0..options.slicing.width {
            if is_edge_pixel(&canvas_reader, &options, base_h, w) {
                // draw shape
                let (v, h) = options.slicing.translate_height_index(base_h);
                let mut current_pixel = canvas_reader[v][h][w];
                let mut active_pixel_face = ActiveSide::North;
            }
        }
    }

    // drawn canvas spans to double the height and width to take into account the subpixel rendering
    // iterate over each pixel, checking for non-sea level colors
    // if color is found, execute path tracing for each color within that range
    // when a shape is discovered, it begins on the top center point (2h, 2w+1)
    // (-1,1), and (0,1) are checked for continuity
    // if no matches found, draw tentative line to (2h+1, 2w+2)
    // recheck with 90 degree rotated pattern
    // If continuity is found, the slope is compared to the slop of the current tentative line
    // If slope matches, continue the line to this point, if not, commit line and create new one
    // when original pixel is reached, draw final line to it and commit
    // if pixel is on (0,0), (0,max), (max,max), or (max,0), midpoint it extended to the edge

    // z level is not needed since shapes should be detected in proper order

    let mut polygons = vec![];

    VectorCanvas {
        background_color_index: options.color_table.sea_level,
        polygons,
    }
}

fn is_edge_pixel(
    canvas: &RwLockReadGuard<Vec<Vec2D<u16>>>,
    options: &RenderOptions,
    base_h: usize,
    w: usize,
) -> bool {
    if base_h == 0
        || w == 0
        || base_h == options.slicing.height - 1
        || w == options.slicing.width - 1
    {
        return true;
    }
    let (v, h) = options.slicing.translate_height_index(base_h);
    let pixel_color = canvas[v][h][w];

    let translations = [
        (base_h - 1, w),
        (base_h, w + 1),
        (base_h + 1, w),
        (base_h, w - 1),
    ];
    for translation in translations {
        let ((v, h), w) = (
            options.slicing.translate_height_index(translation.0),
            translation.1,
        );
        let perimeter_pixel_color = canvas[v][h][w];
        if pixel_color < options.color_table.sea_level {
            if perimeter_pixel_color > pixel_color {
                return true;
            }
        } else if pixel_color > options.color_table.sea_level && perimeter_pixel_color < pixel_color
        {
            return true;
        }
    }
    false
}
