use std::f32::consts::PI as pi32;
use std::f64::consts::PI as pi64;
use crate::geometry::Vertex;

#[derive(Clone)]
pub enum RandPrecision {
    Original,
    Normal,
    High
}

impl RandPrecision {
    /// Pseudorandom number generator taking two seeds
    #[inline(always)]
    #[allow(clippy::approx_constant)]
    pub fn rand(&self, p: &f64, q: &f64) -> f64 {
        let r = match self {
            RandPrecision::Original => (p + 3.14159265) * (q + 3.14159265),
            RandPrecision::High => (p + pi64) * (q + pi64),
            RandPrecision::Normal => (p + pi32 as f64) * (q + pi32 as f64)
        };
        r.fract() + r.fract() - 1.
    }
}

#[inline(always)]
pub fn distance_squared(a: &Vertex, b: &Vertex) -> f64 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    let z = a.z - b.z;
    x * x + y * y + z * z
}

#[derive(Clone)]
pub(crate) struct SeedGenerator {
    pub pre1: RandPrecision,
    pub pre2: RandPrecision,
    pub pre3: RandPrecision,
    pub pre4: RandPrecision,
}

impl SeedGenerator {
    pub fn new(config: &str) -> Self {
        let mut p1 = RandPrecision::Original;
        let mut p2 = RandPrecision::Original;
        let mut p3 = RandPrecision::Original;
        let mut p4 = RandPrecision::Original;

        // config is expected to be 4 characters, but we will make assumptions if otherwise
        for (i, c) in config.chars().enumerate() {
            let precision = match c {
                'n' => RandPrecision::Normal,
                'h' => RandPrecision::High,
                _ => RandPrecision::Original
            };
            
            match i {
                0 => { p1 = precision; }
                1 => { p2 = precision; }
                2 => { p3 = precision; }
                3 => { p4 = precision; }
                _ => {}
            }
        }
        
        Self {
            pre1: p1,
            pre2: p2,
            pre3: p3,
            pre4: p4,
        }
    }

    pub fn generate(&self, seed: f64) -> RenderSeeds {
        let ss1 = self.pre1.rand(&seed, &seed);
        let ss2 = self.pre2.rand(&ss1, &ss1);
        let ss3 = self.pre3.rand(&ss1, &ss2);
        let ss4 = self.pre4.rand(&ss2, &ss3);

        RenderSeeds {
            ss1,
            ss2,
            ss3,
            ss4,
        }
    }
}

#[derive(Clone)]
pub(crate) struct RenderSeeds {
    pub ss1: f64,
    pub ss2: f64,
    pub ss3: f64,
    pub ss4: f64,
}
