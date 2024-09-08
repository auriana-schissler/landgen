use crate::geometry::Vertex;
use crate::projection::Projector;
use crate::render::slicing::Slicing;
use crate::render::RenderOptions;
use crate::terrain::LatLong;
use std::f64::consts::PI;

pub struct Conical {
    slicing: Slicing,
    slice_id: u8,
    i_height: i32,
    i_width: i32,
    scale: f64,
    cp: LatLong,
    scaled_height: f64,
    k1: f64,
    c: f64,
    y2: f64,
}

impl Conical {
    pub fn create(slice_id: u8, options: &RenderOptions) -> Box<dyn Projector> {
        let cp = options.center_point.clone();
        let k1 = 1. / cp.lat_sin;
        let c = k1 * k1;
        Box::new(Self {
            slice_id,
            i_height: options.slicing.height as i32,
            i_width: options.slicing.width as i32,
            scaled_height: options.slicing.height as f64 * options.scale,
            k1,
            c,
            y2: (c * (1. - (cp.latitude / k1).sin()) / (1. + (cp.latitude / k1).sin())).sqrt(),
            scale: options.scale,
            slicing: options.slicing.clone(),
            cp: options.center_point.clone(),
        })
    }
}

impl Projector for Conical {
    fn pixel_to_coordinate(&self, h: usize, w: usize) -> Option<Vertex> {
        let real_h = self.slicing.get_absolute_height(self.slice_id, h);
        let x = (2 * w as i32 - self.i_width) as f64 / self.scaled_height;
        let y = (2 * real_h as i32 - self.i_height) as f64 / self.scaled_height + self.y2;
        let zz = x * x + y * y;
        let mut theta1 = if zz == 0. { 0. } else { self.k1 * x.atan2(y) };
        if (-PI..=PI).contains(&theta1) {
            theta1 += self.cp.longitude - 0.5 * PI;
            let theta2 = self.k1 * ((zz - self.c) / (zz + self.c)).asin();
            if (-PI..=PI).contains(&(theta2 * 2.)) {
                let cos2 = theta2.cos();
                return Some(Vertex::from_point(
                    theta1.cos() * cos2,
                    theta2.sin(),
                    -theta1.sin() * cos2,
                ));
            }
        }
        None
    }

    fn get_subdivision_depth(&self, _: usize) -> u8 {
        if self.scale < 1. {
            (3.0 * self.scaled_height.log2() + 6.0 + 1.5 / self.scale) as u8
        } else {
            3 * self.scaled_height.log2() as u8 + 6
        }
    }
}
