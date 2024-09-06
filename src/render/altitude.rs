use crate::geometry::{side_check, Vertex};
use crate::math::{distance_squared};
use crate::render::ThreadState;
use std::f64::consts::PI;
use std::mem;

// planet1() & planet()
pub fn calc_altitude(state: &mut ThreadState, p: &Vertex) -> f64 {
    let (mut tetra, mut subdivision_depth) = if p.exists_within(&state.cached_tetra) {
        (
            state.cached_tetra.clone(),
            state.starting_subdivision_depth - 5,
        )
    } else {
        (state.base_tetra.clone(), state.starting_subdivision_depth)
    };

    let mut x1;
    let mut y1;
    let mut z1;
    let mut l1;
    let mut tmp;
    let mut e = Vertex::new();

    while subdivision_depth > 0 {
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
                1 => { // a c b d
                    mem::swap(&mut tetra.b, &mut tetra.c);
                }
                2 => { // a d b c
                    mem::swap(&mut tetra.b, &mut tetra.d);
                    mem::swap(&mut tetra.c, &mut tetra.d);
                }
                3 => { // b c a d
                    mem::swap(&mut tetra.a, &mut tetra.c);
                    mem::swap(&mut tetra.b, &mut tetra.c);
                }
                4 => { // b d a c
                    mem::swap(&mut tetra.a, &mut tetra.c);
                    mem::swap(&mut tetra.a, &mut tetra.d);
                    mem::swap(&mut tetra.a, &mut tetra.b);
                }
                5 => { // c d a b
                    mem::swap(&mut tetra.a, &mut tetra.c);
                    mem::swap(&mut tetra.b, &mut tetra.d);
                }
                _ => {
                    unreachable!()
                }
            }
            continue;
        }

        if subdivision_depth == state.starting_subdivision_depth - 5 {
            state.cached_tetra = tetra.clone();
        }

        e.seed = state.options.seed_gen.pre1.rand(&tetra.a.seed, &tetra.b.seed); // ab is longest, so cut ab
        let es1 = state.options.seed_gen.pre1.rand(&e.seed, &e.seed);
        let es2 = 0.5 + 0.1 * state.options.seed_gen.pre2.rand(&es1, &es1); // find cut point
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
            e.x = 0.5 * (tetra.a.x + tetra.b.x);
            e.y = 0.5 * (tetra.a.y + tetra.b.y);
            e.z = 0.5 * (tetra.a.z + tetra.b.z);
        }

        /* new altitude is: */
        if state.options.delta_map.is_some() && lab > state.options.delta_map.unwrap() {
            /* use map height */
            let l = (e.x * e.x + e.y * e.y + e.z * e.z).sqrt();
            let xx = f64::atan2(e.x, e.z) * 23.5 / PI + 23.5;
            let yy = (e.y / l).asin() * 23.0 / PI + 11.5;

            e.altitude = state.search_map[(xx + 0.5).floor() as usize][(yy + 0.5).floor() as usize]
                as f64
                / 80.0;
        } else {
            if lab > 1.0 {
                lab = f64::powf(lab, 0.5);
            }
            /* decrease contribution for very long distances */
            e.altitude = 0.5 * (tetra.a.altitude + tetra.b.altitude) // average of end points
                + e.seed * state.options.alt_diff_weight * f64::powf((tetra.a.altitude - tetra.b.altitude).abs(), state.options.alt_diff_power)
                // plus contribution for altitude diff
                + es1 * state.options.distance_weight * f64::powf(lab, state.options.distance_power);
            // plus contribution for distance
        }

        /* calculate approximate rain shadow for new point */
        if e.altitude <= 0.0 || !(state.options.calculate_rainfall || state.options.show_biomes) {
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

            let z2 = (p.x * z1 - p.z * x1) / tmp;
            if lab > 0.04 {
                e.rain_shadow = (tetra.a.rain_shadow + tetra.b.rain_shadow
                    - (PI * state.options.light.longitude / 180.0).cos() * z2 / l1)
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

        subdivision_depth -= 1;
        if side_check(&ea, &ec, &ed, &ep) {
            // point is inside acde
            mem::swap(&mut tetra.a, &mut tetra.c);
            mem::swap(&mut tetra.b, &mut tetra.d);
            tetra.d = e;
        } else {
            // point is inside bcde
            mem::swap(&mut tetra.a, &mut tetra.c);
            mem::swap(&mut tetra.b, &mut tetra.d);
            mem::swap(&mut tetra.c, &mut tetra.d);
            tetra.d = e;
        }
        continue;
    }

    match state.options.shading_level {
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
            let y2 = y1 * tmp - (p.x * p.y * x1 + p.y * p.z * z1) / tmp;
            let z2 = (p.x * z1 - x1 * p.z) / tmp;

            state.shade = ((-(PI * state.options.light.longitude / 180.0).sin() * y2
                - (PI * state.options.shading_level as f64 / 180.0).cos() * z2)
                / l1
                * 48.0
                + 128.0)
                .clamp(10., 255.) as u8;
            if state.options.shading_level == 2
                && (tetra.a.altitude + tetra.b.altitude + tetra.c.altitude + tetra.d.altitude) < 0.0
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
                l1 = 5.0 * (x1 * x1 + y1 * y1 + z1 * z1).sqrt();
                x1 += p.x * l1;
                y1 += p.y * l1;
                z1 += p.z * l1;
            }
            l1 = (x1 * x1 + y1 * y1 + z1 * z1).sqrt();
            if l1 == 0.0 {
                l1 = 1.0;
            }
            let x2 = (PI * state.options.light.longitude / 180.0 - 0.5 * PI).cos()
                * (PI * state.options.light.latitude / 180.0).cos();
            let y2 = -(PI * state.options.light.latitude / 180.0).sin();
            let z2 = -(PI * state.options.light.longitude / 180.0 - 0.5 * PI).sin()
                * (PI * state.options.light.latitude / 180.0).cos();

            state.shade =
                ((x1 * x2 + y1 * y2 + z1 * z2) / l1 * 170.0 + 10.0).clamp(10., 255.) as u8;
        }
        _ => {}
    }
    state.rain_shadow = 0.25
        * (tetra.a.rain_shadow + tetra.b.rain_shadow + tetra.c.rain_shadow + tetra.d.rain_shadow);
    0.25 * (tetra.a.altitude + tetra.b.altitude + tetra.c.altitude + tetra.d.altitude)
}
