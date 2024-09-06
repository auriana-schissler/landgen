use crate::geometry::Vertex;
use crate::render::color::render_pixel;
use crate::render::{commit_render_data, RenderState, ThreadState};
use chrono::prelude::*;
use std::f64::consts::PI;
use std::sync::Arc;

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

    let k = (0.5 + 0.5 * cp.latitude * scaled_width / PI) as i32;
    for h in 0..state.local_height {
        let real_h = state.options.slicing.get_absolute_height(thread_id, h) as i32;
        let y = (2 * (real_h - k) - i_height) as f64 / scaled_width * PI;

        if 2. * y.abs() > PI {
            for w in 0..width {
                state.canvas[h][w] = state.color_table.back;
                if state.options.shading_level > 0 {
                    state.shading[h][w] = 255;
                }
            }
        } else {
            let cos2 = y.cos();
            if cos2 > 0. {
                let scale1 = scaled_width / (f_height * cos2 * PI);
                state.starting_subdivision_depth = (scale1 * f_height).log2() as u8 * 3 + 3;

                for w in 0..width {
                    let theta1 = cp.longitude - 0.5 * PI
                        + PI * (2 * w - width) as f64 / scaled_width;
                    let world_point =
                        Vertex::from_point(theta1.cos() * cos2, y.sin(), -theta1.sin() * cos2);
                    render_pixel(&mut state, &world_point, w, h);
                }
            }
        }
        if h > 0 && h % 100 == 0 {
            let millis = (Utc::now() - time).num_milliseconds();
            let pixels_per_second = (h * width) as i64 / millis * 1000;
            println!("Thread {thread_id} completed line {h} - {pixels_per_second}pps",);
        }
    }
    commit_render_data(thread_id, state, render_state.clone());
}
