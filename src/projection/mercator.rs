use crate::geometry::Vertex;
use crate::render::color::render_pixel;
use crate::render::{commit_render_data, RenderState, ThreadState};
use std::f64::consts::PI;
use std::sync::Arc;
use chrono::prelude::*;

pub fn render(thread_id: u8, render_state: Arc<RenderState>) {
    let mut state = ThreadState::new(thread_id, render_state.options.clone());

    let i_height = state.options.slicing.height as i32;
    let f_height = state.options.slicing.height as f64;
    let width = state.options.slicing.width;
    let f_width = width as f64;
    let scale = state.options.scale;
    let cp = state.options.center_point.clone();
    let scaled_width = f_width * scale;
    
    let time = Utc::now();

    let mut y = cp.lat_sin;
    y = (1. + y) / (1. - y);
    let k = (0.25 * y.ln() * scaled_width / PI + 0.5) as i32;
    for h in 0..state.local_height {
        let real_h = state.options.slicing.get_absolute_height(thread_id, h);
        y = (2 * (real_h as i32 - k) - i_height) as f64 * 2. * PI / scaled_width;
        y = y.exp();
        y = (y - 1.) / (y + 1.);
        let scale1 = scaled_width / (f_height * (1. - y * y).sqrt() * PI);
        let cos2 = (1. - y * y).sqrt();
        state.starting_subdivision_depth = 3 * (scale1 * f_height).log2() as u8 + 3;
        
        for w in 0..width {
            let theta1 = cp.longitude - 0.5 * PI + PI * (2. * w as f64 - f_width) / scaled_width;
            let world_point = Vertex::from_point(theta1.cos() * cos2, y, -theta1.sin() * cos2);
            render_pixel(&mut state, &world_point, w, h);
        }
        if h > 0 && h % 100 == 0 { 
            let millis = (Utc::now() - time).num_milliseconds();
            let pixels_per_second = (h * width) as i64 / millis * 1000;
            println!("Thread {thread_id} completed line {h} - {pixels_per_second}pps", );
        }
    }
    commit_render_data(thread_id, state, render_state.clone());
}
