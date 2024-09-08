use crate::geometry;
use crate::geometry::Vertex;
use crate::projection::Projector;
use crate::render::slicing::Slicing;
use crate::render::RenderOptions;
use crate::terrain::LatLong;

pub struct Icosahedral {
    slicing: Slicing,
    slice_id: u8,
    i_height: i32,
    i_width: i32,
    cp: LatLong,
    scaled_height: f64,
    scaled_width: f64,
    sq3: f64,
    l1: f64,
    l2: f64,
    s: f64,
}

impl Icosahedral {
    pub fn create(slice_id: u8, options: &RenderOptions) -> Box<dyn Projector> {
        Box::new(Self {
            slice_id,
            i_height: options.slicing.height as i32,
            i_width: options.slicing.width as i32,
            scaled_height: options.slicing.height as f64 * options.scale,
            scaled_width: options.slicing.width as f64 * options.scale,
            slicing: options.slicing.clone(),
            cp: options.center_point.clone(),
            sq3: 3.0_f64.sqrt(),
            l1: 10.812317,  /* theoretically 10.9715145571469; */
            l2: -52.622632, /* theoretically -48.3100310579607; */
            s: 55.6,
        })
    }
}

impl Projector for Icosahedral {
    fn pixel_to_coordinate(&self, h: usize, w: usize) -> Option<Vertex> {
        let real_h = self.slicing.get_absolute_height(self.slice_id, h) as i32;
        let x0 = 198.0 * (2 * w as i32 - self.i_width) as f64 / self.scaled_width - 36.0;
        let y0 = 198.0 * (2 * real_h - self.i_height) as f64 / self.scaled_width
            - self.cp.latitude / 1.0_f64.to_radians();

        let y3 = y0 / self.sq3;
        let (lat, long) = match y3 {
            ..-18.0 => match (x0 - y3, x0 + y3) {
                (..144.0, 36.0..) => (-self.l2, 90.0),
                (..72.0, -36.0..) => (-self.l2, 18.0),
                (..0.0, -108.0..) => (-self.l2, -54.0),
                (..-72.0, -180.0..) => (-self.l2, -126.0),
                (..-144.0, -252.0..) => (-self.l2, -198.0),
                (_, _) => (500.0, 0.),
            },
            -18.0..=18.0 => match (x0 - y3, x0 + y3) {
                (..144.0, 108.0..) => (-self.l1, 126.0),
                (..72.0, 36.0..) => (-self.l1, 54.0),
                (..0.0, -36.0..) => (-self.l1, -18.0),
                (..-72.0, -108.0..) => (-self.l1, -90.0),
                (..-144.0, -180.0..) => (-self.l1, -162.0),
                (72.0.., ..108.0) => (self.l1, 90.0),
                (0.0.., ..36.0) => (self.l1, 18.0),
                (-72.0.., ..-36.0) => (self.l1, -54.0),
                (-144.0.., ..-108.0) => (self.l1, -126.0),
                (-216.0.., ..-180.0) => (self.l1, -198.0),
                (_, _) => (500.0, 0.0),
            },
            18.0.. => match (x0 + y3, x0 - y3) {
                (..180.0, 72.0..) => (self.l2, 126.0),
                (..108.0, 0.0..) => (self.l2, 54.0),
                (..36.0, -72.0..) => (self.l2, -18.0),
                (..-36.0, -144.0..) => (self.l2, -90.0),
                (..-108.0, -216.0..) => (self.l2, -162.0),
                (_, _) => (500., 0.),
            },
            _ => (500.0, 0.0),
        };

        if lat <= 400.0 {
            let mut x = (x0 - long) / self.s;
            let mut y = (y0 + lat) / self.s;

            let point =
                LatLong::new_with_trig(lat.to_radians(), long.to_radians() - self.cp.longitude);

            let zz = (1.0 / (1.0 + x * x + y * y)).sqrt();
            x *= zz;
            y *= zz;
            let z = (1.0 - x * x - y * y).sqrt();
            Some(geometry::common_vertex_from_point(&point, &x, &y, &z))
        } else {
            None
        }
    }

    fn get_subdivision_depth(&self, _: usize) -> u8 {
        3 * self.scaled_height.log2() as u8 + 6
    }
}
