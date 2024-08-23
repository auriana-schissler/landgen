use crate::geometry::Vertex;
use crate::render::color::render_pixel;
use crate::render::{RenderState, ThreadState};
use std::rc::Rc;
use std::sync::Arc;
use chrono::prelude::*;

pub fn render(thread_id: usize, render_state: Arc<RenderState>) {
    let options = Rc::new(render_state.options.clone());
    let mut thread_state = ThreadState::new(thread_id, options.clone());

    let height = options.height as f64;
    let width = options.width as f64;
    let scale = options.scale;
    let p = &options.center_point;

    let time = Utc::now();
    
    thread_state.starting_subdivision_depth = (3 * (scale * height).log2() as u32 + 6) as u8;

    for h in 0..thread_state.local_height {
        for w in 0..options.width as usize {
        let real_h = h + thread_state.starting_line;
        let x = (2. * w as f64 - width) / (height * scale);
        let y = (2 * real_h as i32 - height as i32) as f64 / (height * scale);
            
            if x * x + y * y > 1. {
                thread_state.canvas[h][w] = render_state.color_table.back;
                if options.shading_level > 0 {
                    thread_state.shading[h][w] = 255;
                }
            } else {
                let z = (1. - x * x - y * y).sqrt();

                let world_point = Vertex::from_point(
                    p.long_cos * x + p.long_sin * p.lat_sin * y + p.long_sin * p.lat_cos * z,
                    p.lat_cos * y - p.lat_sin * z,
                    -p.long_sin * x + p.long_cos * p.lat_sin * y + p.long_cos * p.lat_cos *z
                );
                render_pixel(&mut thread_state, &world_point, w, h);
            }            
        }
        if h > 0 && h % 100 == 0 {
            let millis = (Utc::now() - time).num_milliseconds();
            let pixels_per_second = (h as i64 * options.width as i64) / millis * 1000;
            println!("Thread {thread_id} completed line {h} - {pixels_per_second}pps", );
        }
    }
    render_state.canvas.write().unwrap()[thread_id] = thread_state.canvas;
}
