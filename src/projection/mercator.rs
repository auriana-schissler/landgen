use crate::render::slicing::Slicing;
use crate::render::RenderOptions;
use crate::terrain::LatLong;
use std::f64::consts::PI;
use crate::geometry::Vertex;
use crate::projection::Projector;

pub struct Mercator {
    slicing: Slicing,
    slice_id: u8,
    i_height: i32,
    f_height: f64,
    f_width: f64,
    cp: LatLong,
    scaled_width: f64,
    k: i32,
}

impl Mercator {
    pub fn create(slice_id: u8, options: &RenderOptions) -> Box<dyn Projector> {
        let cp = options.center_point.clone();
        Box::new(Self {
            slice_id,
            i_height: options.slicing.height as i32,
            f_height: options.slicing.height as f64,
            f_width: options.slicing.width as f64,
            scaled_width: options.slicing.width as f64 * options.scale,
            slicing: options.slicing.clone(),
            k: (0.25
                * ((1. + cp.lat_sin) / (1. - cp.lat_sin)).ln()
                * options.slicing.width as f64
                * options.scale
                / PI
                + 0.5) as i32,
            cp: options.center_point.clone(),
        })
    }

    fn get_y(&self, h: usize) -> f64 {
        let real_h = self.slicing.get_absolute_height(self.slice_id, h);
        let mut y =
            (2 * (real_h as i32 - self.k) - self.i_height) as f64 * 2. * PI / self.scaled_width;
        y = y.exp();
        (y - 1.) / (y + 1.)
    }
}

impl Projector for Mercator {
    fn pixel_to_coordinate(&self, h: usize, w: usize) -> Option<Vertex> {
        let y = self.get_y(h);
        let cos2 = (1. - y * y).sqrt();
        let theta1 =
            self.cp.longitude - 0.5 * PI + PI * (2.0 * w as f64 - self.f_width) / self.scaled_width;
        Some(Vertex::from_point(
            theta1.cos() * cos2,
            y,
            -theta1.sin() * cos2,
        ))
    }

    fn get_subdivision_depth(&self, h: usize) -> u8 {
        let y = self.get_y(h);
        let cos2 = (1. - y * y).sqrt();
        let scale1 = self.scaled_width / (self.f_height * cos2 * PI);
        3 * (scale1 * self.f_height).log2() as u8 + 3
    }
}