use crate::geometry::Vertex;
use crate::render::altitude::calc_altitude;
use crate::render::ThreadState;
use crate::terrain::biomes;

// planet0()
pub fn render_pixel(thread_state: &mut ThreadState, p: &Vertex, h: usize, w: usize) {
    let options = thread_state.options.clone();
    let mut alt: f64 = calc_altitude(thread_state, p);

    // calculate temperature based on altitude and latitude
    // scale: -0.1 to 0.1 corresponds to -30 to +30 degrees Celsius
    let sun: f64 = (1. - p.y * p.y).sqrt(); //approximate amount of sunlight at latitude ranged from 0.1 to 1.1

    let temp = if alt < 0. {
        sun / 8. + alt * 0.3 // deep water colder
    } else {
        sun / 8. - alt * 1.2 // high altitudes colder
    };

    if options.use_temperature {
        alt = temp - 0.05;
    }

    // calculate rainfall based on temperature and latitude
    // rainfall approximately proportional to temperature but reduced
    //      near horse latitudes (+/- 30 degrees, y=0.5) and reduced for rain shadow
    let mut y2: f64 = p.y.abs() - 0.5;
    let mut rain = temp * 0.65 + 0.1 - 0.011 / (y2 * y2 + 0.1);
    rain += 0.03 * thread_state.rain_shadow;
    if rain < 0.0 {
        rain = 0.0;
    }

    if options.calculate_rainfall {
        alt = rain - 0.02;
    }

    // non-linear scaling to make flatter near sea level
    if options.use_nonlinear_altitude_scaling {
        alt = alt * alt * alt * 300.0;
    }

    // store height for heightfield
    if options.generate_heightfield {
        thread_state.heightfield[h][w] = (10_000_000.0 * alt) as i32;
    }

    y2 = p.y.powi(8);

    let color_table = &thread_state.options.color_table;

    let color = if options.show_biomes {
        let tt = ((rain * 300.0 - 9.0) as i32).clamp(0, 44) as u8;
        let rr = ((temp * 300.0 + 10.0) as i32).clamp(0, 44) as u8;
        let bio = biomes[tt as usize][rr as usize] as u16;
        if alt <= 0.0 {
            let depth_level = (-10. * alt).min(1.);
            let c = (color_table.sea_depth as f64 * depth_level) as u16;
            color_table.sea_level - c
        } else {
            bio - 64 + color_table.lowest_land // from LAND+2 to LAND+23
        }
    } else if alt <= 0. {
        // if below sea level then
        let lci = options.latitude_color_intensity as f64;
        if options.use_latitude_coloring && (y2 + alt) >= (1.0 - 0.02 * lci * lci) {
            color_table.highest_land // icecap if close to poles
        } else {
            let depth_level = (-10. * alt).min(1.);
            let c = (color_table.sea_depth as f64 * depth_level) as u16;
            color_table.sea_level - c
        }
    } else {
        if options.use_latitude_coloring {
            alt += 0.1 * options.latitude_color_intensity as f64 * y2; // altitude adjusted with latitude
        }
        if alt >= 0.1 {
            // if high then
            color_table.highest_land
        } else {
            let altitude = (10.0 * alt).min(1.);
            let c = color_table.land_height as f64 * altitude;
            color_table.lowest_land + c as u16
        }
    };

    thread_state.canvas[h][w] = color;
    if thread_state.shade > 0 {
        thread_state.shading[h][w] = thread_state.shade;
    }

    // store (x,y,z) coordinates for grid drawing
    // if vgrid != 0.0 {
    //     xxx[i][j] = x;
    //     zzz[i][j] = z;
    // }
    // if hgrid != 0.0 || vgrid != 0.0 {
    //     yyy[i][j] = y;
    // }
    //
}
