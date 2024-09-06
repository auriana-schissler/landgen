use crate::render::RenderOptions;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub altitude: f64,
    pub seed: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rain_shadow: f64,
}

impl Vertex {
    pub fn new() -> Self {
        Self {
            altitude: 0.0,
            seed: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rain_shadow: 0.0,
        }
    }

    pub fn from_point(x: f64, y: f64, z: f64) -> Self {
        Self {
            altitude: 0.0,
            seed: 0.0,
            x,
            y,
            z,
            rain_shadow: 0.0,
        }
    }

    #[inline(always)]
    pub fn sub(&self, rhs: &Self) -> Vertex {
        Vertex::from_point(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }

    #[inline(always)]
    pub fn exists_within(&self, t: &Tetra) -> bool {
        let ab = t.b.sub(&t.a);
        let ac = t.c.sub(&t.a);
        let ad = t.d.sub(&t.a);
        let ap = self.sub(&t.a);

        side_check(&ad, &ab, &ac, &ap) &&     // p is on same side of abc as d
        side_check(&ac, &ab, &ad, &ap) &&    // p is on same side of abd as c
        side_check(&ab, &ad, &ac, &ap) && {       // p is on same side of acd as b
            let ba = Vertex::from_point(-ab.x, -ab.y, -ab.z);
            let bc = t.c.sub(&t.b);
            let bd = t.d.sub(&t.b);
            let bp = ap.sub(&t.b);  // self

            // p is on same side of bcd as a. Hence, p is inside cached tetrahedron, so we start from there
            side_check(&ba, &bc, &bd, &bp)
        }
    }
}

#[inline(always)]
pub fn side_check(s1: &Vertex, s2: &Vertex, s3: &Vertex, s4: &Vertex) -> bool {
    side_subcheck(s1, s2, s3) * side_subcheck(s4, s2, s3) > 0.
}

#[inline(always)]
pub fn side_subcheck(s1: &Vertex, s2: &Vertex, s3: &Vertex) -> f64 {
    s1.x * s2.y * s3.z + s3.x * s1.y * s2.z + s2.x * s3.y * s1.z
        - s3.x * s2.y * s1.z
        - s2.x * s1.y * s3.z
        - s1.x * s3.y * s2.z
}

#[derive(Clone)]
pub struct Tetra {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
    pub d: Vertex,
}

impl Tetra {
    pub fn new() -> Self {
        Self::with_points(Vertex::new(), Vertex::new(), Vertex::new(), Vertex::new())
    }

    pub fn with_points(a: Vertex, b: Vertex, c: Vertex, d: Vertex) -> Self {
        Self { a, b, c, d }
    }

    #[inline(always)]
    pub fn sub(&self, rhs: &Self) -> Tetra {
        Tetra {
            a: self.a.sub(&rhs.a),
            b: self.b.sub(&rhs.b),
            c: self.c.sub(&rhs.c),
            d: self.d.sub(&rhs.d),
        }
    }
}

pub fn create_base_tetra(options: &RenderOptions) -> Tetra {
    Tetra {
        a: Vertex {
            x: -3.0_f64.sqrt() - 0.20,
            y: -3.0_f64.sqrt() - 0.22,
            z: -3.0_f64.sqrt() - 0.23,
            seed: options.seeds.ss1,
            altitude: options.initial_altitude,
            rain_shadow: 0.,
        },
        b: Vertex {
            x: -3.0_f64.sqrt() - 0.19,
            y: 3.0_f64.sqrt() + 0.18,
            z: 3.0_f64.sqrt() + 0.17,
            seed: options.seeds.ss2,
            altitude: options.initial_altitude,
            rain_shadow: 0.,
        },
        c: Vertex {
            x: 3.0_f64.sqrt() + 0.21,
            y: -3.0_f64.sqrt() - 0.24,
            z: 3.0_f64.sqrt() + 0.15,
            seed: options.seeds.ss3,
            altitude: options.initial_altitude,
            rain_shadow: 0.,
        },
        d: Vertex {
            x: 3.0_f64.sqrt() + 0.24,
            y: 3.0_f64.sqrt() + 0.22,
            z: -3.0_f64.sqrt() - 0.25,
            seed: options.seeds.ss4,
            altitude: options.initial_altitude,
            rain_shadow: 0.,
        },
    }
}