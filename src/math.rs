use crate::geometry::Vertex;

/// Pseudorandom number generator taking two seeds
#[inline(always)]
#[allow(clippy::approx_constant)]
pub fn rand_low(p: f64, q: f64) -> f64 {
    let r: f64 = (p + 3.14159265) * (q + 3.14159265);   // Inaccurate pi is used for render compat
    r.fract() + r.fract() - 1.
}

#[test]
fn test_rand_accuracy() {
    assert_eq!(rand_low(1., 1.), -0.6944206429319593);
}

#[inline(always)]
pub fn distance_squared(a: &Vertex, b: &Vertex) -> f64 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    let z = a.z - b.z;
    x * x + y * y + z * z
}
