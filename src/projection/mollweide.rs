use crate::geometry::Vertex;
use crate::render::color::render_pixel;
use crate::render::{RenderState, ThreadState};
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Arc;
use chrono::Utc;

pub fn render(thread_id: usize, render_state: Arc<RenderState>) {
    let options = Rc::new(render_state.options.clone());
    let mut thread_state = ThreadState::new(thread_id, options.clone());
    
    let height = options.height as usize;
    let f_height = options.height as f64;
    let width = options.width;
    let f_width = width as f64;
    let scale = options.scale;

    let time = Utc::now();
    
    for h in 0..thread_state.local_height {
        let real_h = (h + thread_state.starting_line) as i32;
        let y1 = 2. * (2 * real_h - height as i32) as f64 / (f_width * scale);
        if y1.abs() >= 1.0 {
            for w in 0..width as usize {
                thread_state.canvas[h][w] = thread_state.color_table.back;
                if options.shading_level > 0 {
                    thread_state.shading[h][w] = 255;
                }
            }
        } else {
            let zz = (1. - y1 * y1).sqrt();
            let y = 2. / PI * (y1 * zz + y1.asin());
            let cos2 = (1. - y * y).sqrt();
            if cos2 > 0. {
                let scale1 = scale * f_width / (f_height * cos2 * PI);
                thread_state.starting_subdivision_depth = 3 * (scale1 * f_height).log2() as u8 + 3;
                for w in 0..width {
                    let mut theta1 = PI * (2 * w - width) as f64 / (f_width * scale * zz);
                    if theta1.abs() > PI {
                        thread_state.canvas[h][w as usize] = render_state.color_table.back;
                        if options.shading_level > 0 {
                            thread_state.shading[h][w as usize] = 255;
                        }
                    } else {
                        theta1 += -0.5 * PI;
                        let cp = &options.center_point;
                        let x2 = theta1.cos() * cos2;
                        let z2 = -theta1.sin() * cos2;
    
                        let world_point = Vertex::from_point(
                            cp.long_cos * x2
                                + cp.long_sin * cp.lat_sin * y
                                + cp.long_sin * cp.lat_cos * z2,
                            cp.lat_cos * y - cp.lat_sin * z2,
                            -cp.long_sin * x2
                                + cp.long_cos * cp.lat_sin * y
                                + cp.long_cos * cp.lat_cos * z2,
                        );
                        render_pixel(&mut thread_state, &world_point, w as usize, h);
                    }
                }
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
