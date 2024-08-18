use crate::geometry::Vertex;
use crate::render::color::render_pixel;
use crate::render::{RenderState, ThreadState};
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Arc;
use chrono::prelude::*;

pub fn render(thread_id: usize, render_state: Arc<RenderState>) {
    let options = Rc::new(render_state.options.clone());
    let mut thread_state = ThreadState::new(thread_id, options.clone());

    let height = options.height as usize;
    let width = options.width as f64;
    let scale = options.scale;
    let longitude = options.center_point.longitude;
    
    let time = Utc::now();

    let mut y = options.center_point.lat_sin;
    y = (1. + y) / (1. - y);
    let k = (0.25 * y.ln() * width * scale / PI + 0.5) as i32;
    for h in 0..thread_state.local_height {
        let real_h = h + thread_state.starting_line;
        y = (2 * (real_h as i32 - k) - height as i32) as f64 * 2. * PI / width / scale;
        y = y.exp();
        y = (y - 1.) / (y + 1.);
        let scale1 = scale * width / height as f64 / (1. - y * y).sqrt() / PI;
        let cos2 = (1. - y * y).sqrt();
        thread_state.starting_subdivision_depth = (3 * (scale1 * height as f64).log2() as u32 + 3) as u8;

        
        for w in 0..options.width as usize {
            let theta1 = longitude - 0.5 * PI + PI * (2. * w as f64 - width) / (width * scale);
            let world_point = Vertex::from_point(theta1.cos() * cos2, y, -theta1.sin() * cos2);
            render_pixel(render_state.clone(), &mut thread_state, &world_point, w, h);
        }
        if h > 0 && h % 100 == 0 { 
            let millis = (Utc::now() - time).num_milliseconds();
            let pixels_per_second = (h as i64 * options.width as i64) / millis * 1000;
            println!("Thread {thread_id} completed line {h} - {pixels_per_second}pps", );
        }
    }
    render_state.canvas.write().unwrap()[thread_id] = thread_state.canvas;
}
