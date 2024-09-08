use crate::geometry::{common_vertex_from_point, Vertex};
use crate::projection::Projector;
use crate::render::slicing::Slicing;
use crate::render::RenderOptions;
use crate::terrain::LatLong;

pub struct Azimuthal {
    slicing: Slicing,
    slice_id: u8,
    i_height: i32,
    i_width: i32,
    cp: LatLong,
    scaled_height: f64,
}

impl Azimuthal {
    pub fn create(slice_id: u8, options: &RenderOptions) -> Box<dyn Projector> {
        Box::new(Self {
            slice_id,
            i_height: options.slicing.height as i32,
            i_width: options.slicing.width as i32,
            scaled_height: options.slicing.height as f64 * options.scale,
            slicing: options.slicing.clone(),
            cp: options.center_point.clone(),
        })
    }
}

impl Projector for Azimuthal {
    fn pixel_to_coordinate(&self, h: usize, w: usize) -> Option<Vertex> {
        let real_h = self.slicing.get_absolute_height(self.slice_id, h);
        let mut x = (2 * w as i32 - self.i_width) as f64 / self.scaled_height;
        let mut y = (2 * real_h as i32 - self.i_height) as f64 / self.scaled_height;
        let mut zz = x * x + y * y;
        if zz <= 4. {
            let z = 1. - 0.5 * zz;
            zz = (1. - 0.25 * zz).sqrt();
            x *= zz;
            y *= zz;
            Some(common_vertex_from_point(&self.cp, &x, &y, &z))
        } else {
            None
        }
    }

    fn get_subdivision_depth(&self, _: usize) -> u8 {
        3 * self.scaled_height.log2() as u8 + 6
    }
}
