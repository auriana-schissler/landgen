use crate::geometry::Vertex;
use crate::render::color::render_pixel;
use crate::render::{RenderState, ThreadState};
use chrono::prelude::*;
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Arc;

pub fn render(thread_id: usize, render_state: Arc<RenderState>) {
    let options = Rc::new(render_state.options.clone());
    let mut thread_state = ThreadState::new(thread_id, options.clone());

    let scale = options.scale;
    let fheight = options.height as f64;
    let fwidth = options.width as f64;
    let p = &options.center_point;

    let time = Utc::now();

    let k = (0.5 + 0.5 * p.latitude * options.width as f64 * scale / PI) as i32;
    for h in 0..thread_state.local_height as i32 {
        let real_h = h + thread_state.starting_line as i32;
        let y = (2 * (real_h - k) - options.height) as f64 / (fwidth * scale) * PI;

        if 2. * y.abs() > PI {
            for w in 0..options.width {
                thread_state.canvas[h as usize][w as usize] = thread_state.color_table.back;
                if options.shading_level > 0 {
                    thread_state.shading[h as usize][w as usize] = 255;
                }
            }
        } else {
            let cos2 = y.cos();
            if cos2 > 0. {
                let scale1 = scale * fwidth / (fheight * cos2 * PI);
                thread_state.starting_subdivision_depth = (scale1 * fheight).log2() as u8 * 3 + 3;

                for w in 0..options.width {
                    let theta1 = p.longitude - 0.5 * PI
                        + PI * (2 * w - options.width) as f64 / (fwidth * scale);
                    let world_point =
                        Vertex::from_point(theta1.cos() * cos2, y.sin(), -theta1.sin() * cos2);
                    render_pixel(&mut thread_state, &world_point, w as usize, h as usize);
                }
            }
        }
        if h > 0 && h % 100 == 0 {
            let millis = (Utc::now() - time).num_milliseconds();
            let pixels_per_second = (h as i64 * options.width as i64) / millis * 1000;
            println!("Thread {thread_id} completed line {h} - {pixels_per_second}pps",);
        }
    }
    render_state.canvas.write().unwrap()[thread_id] = thread_state.canvas;
}
