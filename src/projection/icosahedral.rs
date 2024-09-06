use crate::geometry::Vertex;
use crate::render::color::render_pixel;
use crate::render::{commit_render_data, RenderState, ThreadState};
use crate::terrain::LatLong;
use chrono::prelude::*;
use std::sync::Arc;

pub fn render(thread_id: u8, render_state: Arc<RenderState>) {
    let mut state = ThreadState::new(thread_id, render_state.options.clone());

    let i_height = state.options.slicing.height as i32;
    let f_height = state.options.slicing.height as f64;
    let width = state.options.slicing.width;
    let i_width = state.options.slicing.width as i32;
    let f_width = width as f64;
    let scale = state.options.scale;
    let cp = state.options.center_point.clone();
    let scaled_width = f_width * scale;

    state.starting_subdivision_depth = ((scale * f_height).log2() * 3. + 6.) as u8;
    let time = Utc::now();

    let sq3 = 3.0_f64.sqrt();
    let l1 = 10.812317; /* theoretically 10.9715145571469; */
    let l2 = -52.622632; /* theoretically -48.3100310579607; */
    let s = 55.6;

    for h in 0..state.local_height {
        let real_h = state.options.slicing.get_absolute_height(thread_id, h) as i32;
        for w in 0..width {
            let x0 = 198. * (2 * w as i32 - i_width) as f64 / scaled_width - 36.;
            let y0 = 198. * (2 * real_h - i_height) as f64 / scaled_width
                - cp.latitude / 1.0_f64.to_radians();

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
                state.canvas[h][w] = state.color_table.back;
                if state.options.shading_level > 0 {
                    state.shading[h][w] = 255;
                }
            } else {
                let mut x = (x0 - long) / s;
                let mut y = (y0 + lat) / s;

                let point =
                    LatLong::new_with_trig(lat.to_radians(), long.to_radians() - cp.longitude);

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
                render_pixel(&mut state, &world_point, w, h);
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
