use crate::geometry::Vertex;
use crate::render::color::render_pixel;
use crate::render::{RenderState, ThreadState};
use std::f64::consts::PI;
use std::sync::Arc;

pub fn render(thread_id: usize, render_state: Arc<RenderState>) {
    panic!();
    // let options = render_state.options.clone();
    // let mut thread_state = ThreadState::new(thread_id, render_state.options.clone());
    // let height = options.height as usize;
    // let f_height = options.height as f64;
    // let width = options.width;
    // let f_width = width as f64;
    // let scale = options.scale;
    // 
    // for j in (thread_state.id..height).step_by(render_state.options.render_threads)  {
    //     let y1 = 2. * (2 * j - height) as f64 / (f_width * scale);
    //     if y1.abs() >= 1.0 {
    //         for i in 0..width {
    //             thread_state.canvas[i as usize] = render_state.color_table.back;
    //             if options.shading_level > 0 {
    //                 thread_state.shading[i as usize] = 255;
    //             }
    //         }
    //     } else {
    //         let zz = (1. - y1 * y1).sqrt();
    //         let y = 2. / PI * (y1 * zz + y1.asin());
    //         let cos2 = (1. - y * y).sqrt();
    //         if cos2 > 0. {
    //             let scale1 = scale * f_width / (f_height * cos2 * PI);
    //             thread_state.subdivision_depth = 3 * (scale1 * f_height).log2() as u32 + 3;
    //             for i in 0..width {
    //                 let mut theta1 = PI * (2 * i - width) as f64 / (f_width * scale * zz);
    //                 if theta1.abs() > PI {
    //                     thread_state.canvas[i as usize] = render_state.color_table.back;
    //                     if options.shading_level > 0 {
    //                         thread_state.shading[i as usize] = 255;
    //                     }
    //                 } else {
    //                     theta1 += -0.5 * PI;
    //                     let cp = &options.center_point;
    //                     let x2 = theta1.cos() * cos2;
    //                     let z2 = -theta1.sin() * cos2;
    // 
    //                     let point = Vertex::from_point(
    //                         cp.long_cos * x2
    //                             + cp.long_sin * cp.lat_sin * y
    //                             + cp.long_sin * cp.lat_cos * z2,
    //                         cp.lat_cos * y - cp.lat_sin * z2,
    //                         -cp.long_sin * x2
    //                             + cp.long_cos * cp.lat_sin * y
    //                             + cp.long_cos * cp.lat_cos * z2,
    //                     );
    //                     render_pixel(render_state.clone(), &mut thread_state, &point, i, j);
    //                 }
    //             }
    //         }
    //     }
    // }
}
