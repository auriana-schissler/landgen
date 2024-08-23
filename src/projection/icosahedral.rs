use crate::geometry::Vertex;
use crate::render::color::render_pixel;
use crate::render::{RenderState, ThreadState};
use crate::terrain::LatLong;
use chrono::prelude::*;
use std::rc::Rc;
use std::sync::Arc;

pub fn render(thread_id: usize, render_state: Arc<RenderState>) {
    let options = Rc::new(render_state.options.clone());
    let mut thread_state = ThreadState::new(thread_id, options.clone());

    let scale = options.scale;
    let p = &options.center_point;

    thread_state.starting_subdivision_depth =
        ((scale * options.height as f64).log2() * 3. + 6.) as u8;
    let time = Utc::now();

    let sq3 = 3.0_f64.sqrt();
    let l1 = 10.812317; /* theoretically 10.9715145571469; */
    let l2 = -52.622632; /* theoretically -48.3100310579607; */
    //let l2 = -48.3100310579607;
    let s = 55.6;

    for h in 0..thread_state.local_height as i32 {
        let real_h = h + thread_state.starting_line as i32;
        for w in 0..options.width {
            let x0 = 198. * (2 * w - options.width) as f64 / (options.width as f64 * scale) - 36.;
            let y0 = 198. * (2 * real_h - options.height) as f64 / options.width as f64 / scale
                - p.latitude / 1.0_f64.to_radians();

            let y3 = y0 / sq3;
            let (lat, long) = match y3 {
                ..-18. => match (x0 - y3, x0 + y3) {
                    (..144., 36.0..) => (-l2, 90.),
                    (..72., -36.0..) => (-l2, 18.),
                    (..0., -108.0..) => (-l2, -54.),
                    (..-72., -180.0..) => (-l2, -126.),
                    (..-144., -252.0..) => (-l2, -198.),
                    (_, _) => (500., 0.),
                },
                -18.0..=18.0 => match (x0 - y3, x0 + y3) {
                    (..144., 108.0..) => (-l1, 126.),
                    (..72., 36.0..) => (-l1, 54.),
                    (..0., -36.0..) => (-l1, -18.),
                    (..-72., -108.0..) => (-l1, -90.),
                    (..-144., -180.0..) => (-l1, -162.),
                    (72.0.., ..108.0) => (l1, 90.),
                    (0.0.., ..36.0) => (l1, 18.),
                    (-72.0.., ..-36.0) => (l1, -54.),
                    (-144.0.., ..-108.0) => (l1, -126.),
                    (-216.0.., ..-180.0) => (l1, -198.),
                    (_, _) => (500., 0.),
                },
                18.0.. => match (x0 + y3, x0 - y3) {
                    (..180., 72.0..) => (l2, 126.),
                    (..108., 0.0..) => (l2, 54.),
                    (..36., -72.0..) => (l2, -18.),
                    (..-36., -144.0..) => (l2, -90.),
                    (..-108., -216.0..) => (l2, -162.),
                    (_, _) => (500., 0.),
                },
                _ => (500., 0.),
            };

            if lat > 400. {
                thread_state.canvas[h as usize][w as usize] = thread_state.color_table.back;
                if options.shading_level > 0 {
                    thread_state.shading[h as usize][w as usize] = 255;
                }
            } else {
                let mut x = (x0 - long) / s;
                let mut y = (y0 + lat) / s;

                let point =
                    LatLong::new_with_trig(lat.to_radians(), long.to_radians() - p.longitude);

                let zz = (1. / (1. + x * x + y * y)).sqrt();
                x *= zz;
                y *= zz;
                let z = (1. - x * x - y * y).sqrt();

                let world_point = Vertex::from_point(
                    point.long_cos * x
                        + point.long_sin * point.lat_sin * y
                        + point.long_sin * point.lat_cos * z,
                    point.lat_cos * y - point.lat_sin * z,
                    -point.long_sin * x
                        + point.long_cos * point.lat_sin * y
                        + point.long_cos * point.lat_cos * z,
                );
                render_pixel(&mut thread_state, &world_point, w as usize, h as usize);
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
