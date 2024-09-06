use crate::geometry::Vertex;
use crate::render::color::render_pixel;
use crate::render::{commit_render_data, RenderState, ThreadState};
use chrono::prelude::*;
use std::sync::Arc;

pub fn render(thread_id: u8, render_state: Arc<RenderState>) {
    let mut state = ThreadState::new(thread_id, render_state.options.clone());

    let i_height = state.options.slicing.height as i32;
    let f_height = state.options.slicing.height as f64;
    let width = state.options.slicing.width;
    let i_width = state.options.slicing.width as i32;
    let scale = state.options.scale;
    let cp = state.options.center_point.clone();
    let scaled_height = f_height * scale;

    state.starting_subdivision_depth = if scale < 1. {
        (3. * (scale * f_height).log2() + 6. + 1.5 / scale) as u8
    } else {
        (3 * (scale * f_height).log2() as u32 + 6) as u8
    };
    let time = Utc::now();

    for h in 0..state.local_height {
        let real_h = state.options.slicing.get_absolute_height(thread_id, h);
        for w in 0..width {
            let mut x = (2 * w as i32 - i_width) as f64 / scaled_height;
            let mut y = (2 * real_h as i32 - i_height) as f64 / scaled_height;
            let zz = (1. / (1. + x * x + y * y)).sqrt();
            x *= zz;
            y *= zz;
            let z = (1. - x * x - y * y).sqrt();
            let world_point = Vertex::from_point(
                cp.long_cos * x + cp.long_sin * cp.lat_sin * y + cp.long_sin * cp.lat_cos * z,
                cp.lat_cos * y - cp.lat_sin * z,
                -cp.long_sin * x + cp.long_cos * cp.lat_sin * y + cp.long_cos * cp.lat_cos * z,
            );
            render_pixel(&mut state, &world_point, w, h);
        }
        if h > 0 && h % 100 == 0 {
            let millis = (Utc::now() - time).num_milliseconds();
            let pixels_per_second = (h * width) as i64 / millis * 1000;
            println!("Thread {thread_id} completed line {h} - {pixels_per_second}pps",);
        }
    }
    commit_render_data(thread_id, state, render_state.clone());
}
