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
    let i_width = width as i32;
    let scale = state.options.scale;
    let cp = state.options.center_point.clone();
    let scaled_height = f_height * scale;

    state.starting_subdivision_depth = if scale < 1. {
        (3. * scaled_height.log2() + 6. + 1.5 / scale) as u8
    } else {
        3 * scaled_height.log2() as u8 + 6
    };
    let time = Utc::now();

    let k1 = 1. / cp.lat_sin;
    let c = k1 * k1;
    let y2 = (c * (1. - (cp.latitude / k1).sin()) / (1. + (cp.latitude / k1).sin())).sqrt();

    for h in 0..state.local_height {
        let real_h = state.options.slicing.get_absolute_height(thread_id, h);
        for w in 0..width {
            let x = (2 * w as i32 - i_width) as f64 / scaled_height;
            let mut y = (2 * real_h as i32 - i_height) as f64 / scaled_height + y2;
            let zz = x * x + y * y;
            let mut theta1 = if zz == 0. { 0. } else { k1 * x.atan2(y) };
            if !(-PI..=PI).contains(&theta1) {
                state.canvas[h][w] = state.color_table.back;
                if state.options.shading_level > 0 {
                    state.shading[h][w] = 255;
                }
            } else {
                theta1 += cp.longitude - 0.5 * PI;
                let theta2 = k1 * ((zz - c) / (zz + c)).asin();
                if !(-0.5 * PI..=0.5 * PI).contains(&theta2) {
                    state.canvas[h][w] = state.color_table.back;
                    if state.options.shading_level > 0 {
                        state.shading[h][w] = 255;
                    }
                } else {
                    let cos2 = theta2.cos();
                    y = theta2.sin();
                    let world_point =
                        Vertex::from_point(theta1.cos() * cos2, y, -theta1.sin() * cos2);
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
