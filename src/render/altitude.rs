use crate::geometry::{side_check, Tetra, Vertex};
use crate::math::{distance_squared, rand_low};
use crate::render::ThreadState;
use std::f64::consts::PI;
use std::mem;

// planet1()
pub fn calc_altitude(state: &mut ThreadState, p: &Vertex) -> f64 {
    if p.exists_within(&state.cached_tetra) {
        execute(
            state,
            &mut state.cached_tetra.clone(),
            p,
            state.starting_subdivision_depth - 5,
        )
    } else {
        execute(
            state,
            &mut state.base_tetra.clone(),
            p,
            state.starting_subdivision_depth,
        )
    }
}

// planet()
pub fn execute(
    state: &mut ThreadState,
    tetra: &mut Tetra,
    p: &Vertex,
    subdivision_depth: u8,
) -> f64 {
    let mut x1;
    let mut y1;
    let mut z1;
    let x2;
    let y2;
    let z2;
    let mut l1;
    let mut tmp;
    let mut e = Vertex::new();
    let options = state.options.clone();

    if subdivision_depth > 0 {
        /* make sure ab is longest edge */
        let mut lab = distance_squared(&tetra.a, &tetra.b);
        let lac = distance_squared(&tetra.a, &tetra.c);
        let lad = distance_squared(&tetra.a, &tetra.d);
        let lbc = distance_squared(&tetra.b, &tetra.c);
        let lbd = distance_squared(&tetra.b, &tetra.d);
        let lcd = distance_squared(&tetra.c, &tetra.d);

        let mut maxlength = lab;
        let mut max = 0_u8;
        if lac > maxlength {
            maxlength = lac;
            max = 1;
        }
        if lad > maxlength {
            maxlength = lad;
            max = 2;
        }
        if lbc > maxlength {
            maxlength = lbc;
            max = 3;
        }
        if lbd > maxlength {
            maxlength = lbd;
            max = 4;
        }
        if lcd > maxlength {
            max = 5;
        }

        if (1..=5_u8).contains(&max) {
            match max {
                1 => {  // a c b d             
                    mem::swap(&mut tetra.b, &mut tetra.c);
                },
                2 => {  // a d b c
                    mem::swap(&mut tetra.b, &mut tetra.d);
                    mem::swap(&mut tetra.c, &mut tetra.d);
                }
                3 => { // b c a d
                    mem::swap(&mut tetra.a, &mut tetra.c);
                    mem::swap(&mut tetra.b, &mut tetra.c);
                }
                4 => {  // b d a c
                    mem::swap(&mut tetra.a, &mut tetra.c);
                    mem::swap(&mut tetra.a, &mut tetra.d);
                    mem::swap(&mut tetra.a, &mut tetra.b);
                }
                5 => {  // c d a b
                    mem::swap(&mut tetra.a, &mut tetra.c);
                    mem::swap(&mut tetra.b, &mut tetra.d);
                }
                _ => {
                    unreachable!()
                }
            }
            return execute(state, tetra, p, subdivision_depth);
        }

        if subdivision_depth == state.starting_subdivision_depth - 5 {
            state.cached_tetra = tetra.clone();
        }

        e.seed = rand_low(tetra.a.seed, tetra.b.seed); // ab is longest, so cut ab
        let es1 = rand_low(e.seed, e.seed);
        let es2 = 0.5 + 0.1 * rand_low(es1, es1); // find cut point
        let es3 = 1. - es2;

        if tetra.a.seed < tetra.b.seed {
            e.x = es2 * tetra.a.x + es3 * tetra.b.x;
            e.y = es2 * tetra.a.y + es3 * tetra.b.y;
            e.z = es2 * tetra.a.z + es3 * tetra.b.z;
        } else if tetra.a.seed > tetra.b.seed {
            e.x = es3 * tetra.a.x + es2 * tetra.b.x;
            e.y = es3 * tetra.a.y + es2 * tetra.b.y;
            e.z = es3 * tetra.a.z + es2 * tetra.b.z;
        } else {
            /* as==bs, very unlikely to ever happen */
            e.x = 0.5 * tetra.a.x + 0.5 * tetra.b.x;
            e.y = 0.5 * tetra.a.y + 0.5 * tetra.b.y;
            e.z = 0.5 * tetra.a.z + 0.5 * tetra.b.z;
        }

        /* new altitude is: */
        if options.delta_map.is_some() && lab > options.delta_map.unwrap() {
            /* use map height */
            let l = (e.x * e.x + e.y * e.y + e.z * e.z).sqrt();
            let xx = f64::atan2(e.x, e.z) * 23.5 / PI + 23.5;
            let yy = (e.y / l).asin() * 23.0 / PI + 11.5;

            e.altitude = state.search_map[(xx + 0.5).floor() as usize][(yy + 0.5).floor() as usize]
                as f64
                * 0.1
                / 8.0;
        } else {
            if lab > 1.0 {
                lab = f64::powf(lab, 0.5);
            }
            /* decrease contribution for very long distances */
            e.altitude = 0.5 * (tetra.a.altitude + tetra.b.altitude) // average of end points
                + e.seed * options.alt_diff_weight * f64::powf((tetra.a.altitude - tetra.b.altitude).abs(), options.alt_diff_power)
                // plus contribution for altitude diff
                + es1 * options.distance_weight * f64::powf(lab, options.distance_power);
            // plus contribution for distance
        }

        /* calculate approximate rain shadow for new point */
        if e.altitude <= 0.0 || !(options.calculate_rainfall || options.show_biomes) {
            e.rain_shadow = 0.0;
        } else {
            x1 = 0.5 * (tetra.a.x + tetra.b.x);
            x1 = tetra.a.altitude * (x1 - tetra.a.x) + tetra.b.altitude * (x1 - tetra.b.x);
            y1 = 0.5 * (tetra.a.y + tetra.b.y);
            y1 = tetra.a.altitude * (y1 - tetra.a.y) + tetra.b.altitude * (y1 - tetra.b.y);
            z1 = 0.5 * (tetra.a.z + tetra.b.z);
            z1 = tetra.a.altitude * (z1 - tetra.a.z) + tetra.b.altitude * (z1 - tetra.b.z);
            l1 = (x1 * x1 + y1 * y1 + z1 * z1).sqrt();
            if l1 == 0.0 {
                l1 = 1.0;
            }
            tmp = (1.0 - p.y * p.y).sqrt();
            if tmp < 0.0001 {
                tmp = 0.0001;
            }

            z2 = -p.z / tmp * x1 + p.x / tmp * z1;
            if lab > 0.04 {
                e.rain_shadow = (tetra.a.rain_shadow + tetra.b.rain_shadow
                    - (PI * options.daylight.longitude / 180.0).cos() * z2 / l1)
                    / 3.0;
            } else {
                e.rain_shadow = (tetra.a.rain_shadow + tetra.b.rain_shadow) / 2.0;
            }
        }

        /* find out in which new tetrahedron target point is */
        let ea = tetra.a.sub(&e);
        let ec = tetra.c.sub(&e);
        let ed = tetra.d.sub(&e);
        let ep = p.sub(&e);

        if side_check(&ea, &ec, &ed, &ep) {
            // point is inside acde
            mem::swap(&mut tetra.a, &mut tetra.c);
            mem::swap(&mut tetra.b, &mut tetra.d);
            tetra.d = e;
            execute(state, tetra, p, subdivision_depth - 1)
        } else {
            // point is inside bcde
            mem::swap(&mut tetra.a, &mut tetra.c);
            mem::swap(&mut tetra.b, &mut tetra.d);
            mem::swap(&mut tetra.c, &mut tetra.d);
            tetra.d = e;
            execute(state, tetra, p, subdivision_depth - 1)
        }
    } else {
        // subdivision_level == 0
        match options.shading_level {
            1 | 2 => {
                /* bump map */
                x1 = 0.25 * (tetra.a.x + tetra.b.x + tetra.c.x + tetra.d.x);
                x1 = tetra.a.altitude * (x1 - tetra.a.x)
                   + tetra.b.altitude * (x1 - tetra.b.x)
                   + tetra.c.altitude * (x1 - tetra.c.x)
                   + tetra.d.altitude * (x1 - tetra.d.x);
                y1 = 0.25 * (tetra.a.y + tetra.b.y + tetra.c.y + tetra.d.y);
                y1 = tetra.a.altitude * (y1 - tetra.a.y)
                   + tetra.b.altitude * (y1 - tetra.b.y)
                   + tetra.c.altitude * (y1 - tetra.c.y)
                   + tetra.d.altitude * (y1 - tetra.d.y);
                z1 = 0.25 * (tetra.a.z + tetra.b.z + tetra.c.z + tetra.d.z);
                z1 = tetra.a.altitude * (z1 - tetra.a.z)
                   + tetra.b.altitude * (z1 - tetra.b.z)
                   + tetra.c.altitude * (z1 - tetra.c.z)
                   + tetra.d.altitude * (z1 - tetra.d.z);
                l1 = (x1 * x1 + y1 * y1 + z1 * z1).sqrt();
                if l1 == 0.0 {
                    l1 = 1.0;
                }
                tmp = (1.0 - p.y * p.y).sqrt();
                if tmp < 0.0001 {
                    tmp = 0.0001;
                }
                y2 = -p.x * p.y / tmp * x1 + tmp * y1 - p.z * p.y / tmp * z1;
                z2 = -p.z / tmp * x1 + p.x / tmp * z1;

                state.shade = ((-(PI * options.daylight.longitude / 180.0).sin() * y2
                    - (PI * options.shading_level as f64 / 180.0).cos() * z2)
                    / l1
                    * 48.0
                    + 128.0)
                    .clamp(10., 255.) as u8;
                if options.shading_level == 2
                    && (tetra.a.altitude + tetra.b.altitude + tetra.c.altitude + tetra.d.altitude)
                        < 0.0
                {
                    state.shade = 150;
                }
            }
            3 => {
                /* daylight shading */
                let hh = tetra.a.altitude + tetra.b.altitude + tetra.c.altitude + tetra.d.altitude;
                if hh <= 0.0 {
                    /* sea */
                    x1 = p.x;
                    y1 = p.y;
                    z1 = p.z; /* (x1,y1,z1) = normal vector */
                } else {
                    /* add bumpmap effect */
                    x1 = 0.25 * (tetra.a.x + tetra.b.x + tetra.c.x + tetra.d.x);
                    x1 = calc_bump_point(tetra, x1);
                    y1 = 0.25 * (tetra.a.y + tetra.b.y + tetra.c.y + tetra.d.y);
                    y1 = calc_bump_point(tetra, y1);
                    z1 = 0.25 * (tetra.a.z + tetra.b.z + tetra.c.z + tetra.d.z);
                    z1 = calc_bump_point(tetra, z1);
                    l1 = 5.0 * (x1 * x1 + y1 * y1 + z1 * z1).sqrt();
                    x1 += p.x * l1;
                    y1 += p.y * l1;
                    z1 += p.z * l1;
                }
                l1 = (x1 * x1 + y1 * y1 + z1 * z1).sqrt();
                if l1 == 0.0 {
                    l1 = 1.0;
                }
                x2 = (PI * options.daylight.longitude / 180.0 - 0.5 * PI).cos()
                    * (PI * options.daylight.latitude / 180.0).cos();
                y2 = -(PI * options.daylight.latitude / 180.0).sin();
                z2 = -(PI * options.daylight.longitude / 180.0 - 0.5 * PI).sin()
                    * (PI * options.daylight.latitude / 180.0).cos();

                state.shade = ((x1 * x2 + y1 * y2 + z1 * z2) / l1 * 170.0 + 10.0).clamp(10., 255.) as u8;
            }
            _ => {}
        }
        state.rain_shadow = 0.25
            * (tetra.a.rain_shadow
                + tetra.b.rain_shadow
                + tetra.c.rain_shadow
                + tetra.d.rain_shadow);
        0.25 * (tetra.a.altitude + tetra.b.altitude + tetra.c.altitude + tetra.d.altitude)
    }
}

#[inline(always)]
fn calc_bump_point(t: &Tetra, p: f64) -> f64 {
    t.a.altitude * (p - t.a.x)
        + t.b.altitude * (p - t.b.x)
        + t.c.altitude * (p - t.c.x)
        + t.d.altitude * (p - t.d.x)
}
