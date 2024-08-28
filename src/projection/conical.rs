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
    let p = &options.center_point;

    thread_state.starting_subdivision_depth = if scale < 1. {
        (3. * (scale * options.height as f64).log2() + 6. + 1.5 / scale) as u8
    } else {
        (3 * (options.scale * options.height as f64).log2() as u32 + 6) as u8
    };
    let time = Utc::now();

    let k1 = 1. / p.lat_sin;
    let c = k1 * k1;
    let y2 = (c * (1. - (p.latitude / k1).sin()) / (1. + (p.latitude / k1).sin())).sqrt();

    for h in 0..thread_state.local_height as i32 {
        let real_h = h + thread_state.starting_line as i32;
        for w in 0..options.width {
            let x = (2 * w - options.width) as f64 / (options.height as f64 * scale);
            let mut y = (2 * real_h - options.height) as f64 / (options.height as f64 * scale) + y2;
            let zz = x * x + y * y;
            let mut theta1 = if zz == 0. { 0. } else { k1 * x.atan2(y) };
            if theta1 < -PI || theta1 > PI {
                thread_state.canvas[h as usize][w as usize] = thread_state.color_table.back;
                if options.shading_level > 0 {
                    thread_state.shading[h as usize][w as usize] = 255;
                }
            } else {
                theta1 += p.longitude - 0.5 * PI;
                let theta2 = k1 * ((zz - c) / (zz + c)).asin();
                if theta2 > 0.5 * PI || theta2 < -0.5 * PI {
                    thread_state.canvas[h as usize][w as usize] = thread_state.color_table.back;
                    if options.shading_level > 0 {
                        thread_state.shading[h as usize][w as usize] = 255;
                    }
                } else {
                    let cos2 = theta2.cos();
                    y = theta2.sin();
                    let world_point =
                        Vertex::from_point(theta1.cos() * cos2, y, -theta1.sin() * cos2);
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
