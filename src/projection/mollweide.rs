use crate::geometry::Vertex;
use crate::projection::Projector;
use crate::render::slicing::Slicing;
use crate::render::RenderOptions;
use crate::terrain::LatLong;
use std::f64::consts::PI;
use crate::geometry;

pub struct Mollweide {
    slicing: Slicing,
    slice_id: u8,
    i_height: i32,
    f_height: f64,
    i_width: i32,
    cp: LatLong,
    scaled_width: f64,
}

impl Mollweide {
    pub fn create(slice_id: u8, options: &RenderOptions) -> Box<dyn Projector> {
        Box::new(Self {
            slice_id,
            i_height: options.slicing.height as i32,
            f_height: options.slicing.height as f64,
            i_width: options.slicing.width as i32,
            scaled_width: options.slicing.width as f64 * options.scale,
            slicing: options.slicing.clone(),
            cp: options.center_point.clone(),
        })
    }
}

impl Projector for Mollweide {
    fn pixel_to_coordinate(&self, h: usize, w: usize) -> Option<Vertex> {
        let real_h = self.slicing.get_absolute_height(self.slice_id, h) as i32;
        let y1 = 2. * (2 * real_h - self.i_height) as f64 / self.scaled_width;
        if y1.abs() < 1.0 {
            let zz = (1. - y1 * y1).sqrt();
            let y = 2. / PI * (y1 * zz + y1.asin());
            let cos2 = (1. - y * y).sqrt();
            if cos2 > 0. {
                let mut theta1 =
                    PI * (2 * w as i32 - self.i_width) as f64 / (self.scaled_width * zz);
                if theta1.abs() <= PI {
                    theta1 += -0.5 * PI;
                    let x2 = theta1.cos() * cos2;
                    let z2 = -theta1.sin() * cos2;

                    return Some(geometry::common_vertex_from_point(&self.cp, &x2, &y, &z2));
                }
            }
        }
        None
    }

    fn get_subdivision_depth(&self, h: usize) -> u8 {
        let real_h = self.slicing.get_absolute_height(self.slice_id, h) as i32;
        let y1 = 2. * (2 * real_h - self.i_height) as f64 / self.scaled_width;
        let zz = (1. - y1 * y1).sqrt();
        let y = 2. / PI * (y1 * zz + y1.asin());
        let cos2 = (1. - y * y).sqrt();
        let scale1 = self.scaled_width / (self.f_height * cos2 * PI);
        3 * (scale1 * self.f_height).log2() as u8 + 3
    }
}
