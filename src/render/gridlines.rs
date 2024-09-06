use std::sync::Arc;
use crate::render::RenderState;
use crate::util::Vec2D;

pub struct GridLines {
    pub x: Vec2D<f64>,
    pub y: Vec2D<f64>,
    pub z: Vec2D<f64>,
}

impl GridLines {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            x: vec![vec! {0.0; width}; height],
            y: vec![vec! {0.0; width}; height],
            z: vec![vec! {0.0; width}; height],
        }
    }
}

// TODO: Generate lat/long grid lines
pub fn generate_gridlines(state: Arc<RenderState>) {}