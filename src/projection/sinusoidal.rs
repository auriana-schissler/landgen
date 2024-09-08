use crate::geometry::Vertex;
use crate::projection::Projector;
use crate::render::slicing::Slicing;
use crate::render::RenderOptions;
use crate::terrain::LatLong;
use std::f64::consts::PI;

/// BUG: The math of this does not check out properly at float zoom levels, and it needs research into cut sinusoidal algorithms to be fixed
pub struct Sinusoidal {
    slicing: Slicing,
    slice_id: u8,
    i_height: i32,
    f_height: f64,
    f_width: f64,
    scale: f64,
    cp: LatLong,
    scaled_width: f64,
    k: i32,
}

impl Sinusoidal {
    pub fn create(slice_id: u8, options: &RenderOptions) -> Box<dyn Projector> {
        let cp = options.center_point.clone();
        Box::new(Self {
            slice_id,
            i_height: options.slicing.height as i32,
            f_height: options.slicing.height as f64,
            f_width: options.slicing.width as f64,
            scaled_width: options.slicing.width as f64 * options.scale,
            scale: options.scale,
            slicing: options.slicing.clone(),
            k: (cp.latitude * options.slicing.width as f64 * options.scale / PI + 0.5) as i32,
            cp: options.center_point.clone(),
        })
    }
}

impl Projector for Sinusoidal {
    fn pixel_to_coordinate(&self, h: usize, w: usize) -> Option<Vertex> {
        let real_h = self.slicing.get_absolute_height(self.slice_id, h) as i32;
        let y = (2 * (real_h - self.k) - self.i_height) as f64 / self.scaled_width * PI;
        let cos2 = y.cos();
        if cos2 > 0. {
            let l = (12 * w) as f64 / (self.f_width * self.scale);
            let l1 = l * self.scaled_width / 12.;
            let theta2 =
                self.cp.longitude - 0.5 * PI + PI * (2. * l1 - self.f_width) / self.scaled_width;
            let theta1 =
                PI * (2. * w as f64 - self.scaled_width / 12.) / (self.scaled_width * cos2);
            if theta1.abs() <= PI / 1. {
                return Some(Vertex::from_point(
                    (theta1 + theta2).cos() * cos2,
                    y.sin(),
                    -(theta1 + theta2).sin() * cos2,
                ));
            }
        }
        None
    }

    fn get_subdivision_depth(&self, h: usize) -> u8 {
        let real_h = self.slicing.get_absolute_height(self.slice_id, h) as i32;
        let y = (2 * (real_h - self.k) - self.i_height) as f64 / self.scaled_width * PI;
        let scale1 = self.scaled_width / (self.f_height * y.cos() * PI);
        (scale1 * self.f_height).log2() as u8 * 3 + 3
    }
}
