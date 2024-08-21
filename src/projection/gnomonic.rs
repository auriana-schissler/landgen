use crate::geometry::Vertex;
use crate::render::color::render_pixel;
use crate::render::{RenderState, ThreadState};
use chrono::prelude::*;
use std::rc::Rc;
use std::sync::Arc;

pub fn render(thread_id: usize, render_state: Arc<RenderState>) {
    let options = Rc::new(render_state.options.clone());
    let mut thread_state = ThreadState::new(thread_id, options.clone());

    let scale = options.scale;
    let p = &options.center_point;

    thread_state.starting_subdivision_depth = if scale < 1. {
        (3. * (scale * options.height as f64).log2() + 6. + 1.5 / scale) as u8
    } else {
        (3 * (options.scale * options.height as f64).log2() as u32 + 6) as u8
    };
    let time = Utc::now();

    for h in 0..thread_state.local_height as i32 {
        let real_h = h + thread_state.starting_line as i32;
        for w in 0..options.width {
            let mut x = (2 * w - options.width) as f64 / (options.height as f64 * scale);
            let mut y = (2 * real_h - options.height) as f64 / (options.height as f64 * scale);
            let zz = (1. / (1. + x * x + y * y)).sqrt();
            x *= zz;
            y *= zz;
            let z = (1. - x * x - y * y).sqrt();
            let world_point = Vertex::from_point(
                p.long_cos * x + p.long_sin * p.lat_sin * y + p.long_sin * p.lat_cos * z,
                p.lat_cos * y - p.lat_sin * z,
                -p.long_sin * x + p.long_cos * p.lat_sin * y + p.long_cos * p.lat_cos * z,
            );
            render_pixel(&mut thread_state, &world_point, w as usize, h as usize);
        }
        if h > 0 && h % 100 == 0 {
            let millis = (Utc::now() - time).num_milliseconds();
            let pixels_per_second = (h as i64 * options.width as i64) / millis * 1000;
            println!("Thread {thread_id} completed line {h} - {pixels_per_second}pps",);
        }
    }
    render_state.canvas.write().unwrap()[thread_id] = thread_state.canvas;
}
