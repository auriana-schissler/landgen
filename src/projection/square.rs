use crate::geometry::Vertex;
use crate::projection::Projector;
use crate::render::slicing::Slicing;
use crate::render::RenderOptions;
use crate::terrain::LatLong;
use std::f64::consts::PI;

pub struct Square {
    slicing: Slicing,
    slice_id: u8,
    i_height: i32,
    f_height: f64,
    i_width: i32,
    cp: LatLong,
    scaled_width: f64,
    k: i32,
}

impl Square {
    pub fn create(slice_id: u8, options: &RenderOptions) -> Box<dyn Projector> {
        let cp = options.center_point.clone();
        Box::new(Self {
            slice_id,
            i_height: options.slicing.height as i32,
            f_height: options.slicing.height as f64,
            i_width: options.slicing.width as i32,
            scaled_width: options.slicing.width as f64 * options.scale,
            slicing: options.slicing.clone(),
            k: (0.5 + 0.5 * cp.latitude * options.slicing.width as f64 * options.scale / PI) as i32,
            cp: options.center_point.clone(),
        })
    }
}

impl Projector for Square {
    fn pixel_to_coordinate(&self, h: usize, w: usize) -> Option<Vertex> {
        let real_h = self.slicing.get_absolute_height(self.slice_id, h) as i32;
        let y = (2 * (real_h - self.k) - self.i_height) as f64 / self.scaled_width * PI;

        if 2. * y.abs() <= PI {
            let cos2 = y.cos();
            if cos2 > 0. {
                let theta1 = self.cp.longitude - 0.5 * PI
                    + PI * (2 * w as i32 - self.i_width) as f64 / self.scaled_width;
                return Some(Vertex::from_point(
                    theta1.cos() * cos2,
                    y.sin(),
                    -theta1.sin() * cos2,
                ));
            }
        }
        None
    }

    fn get_subdivision_depth(&self, h: usize) -> u8 {
        let real_h = self.slicing.get_absolute_height(self.slice_id, h) as i32;
        let y = (2 * (real_h - self.k) - self.i_height) as f64 / self.scaled_width * PI;
        let cos2 = y.cos();
        let scale1 = self.scaled_width / (self.f_height * cos2 * PI);
        (scale1 * self.f_height).log2() as u8 * 3 + 3
    }
}
