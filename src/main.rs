mod color;
mod file;
mod math;
mod render;
mod terrain;
mod util;
mod geometry;
mod crc;
mod projection;

use clap::Parser;
use std::env;
use std::mem::size_of;

fn main() {
    if size_of::<usize>() < size_of::<u64>() {
        eprint!("This program requires 64-bit processing and is not compatible on this processor architecture. ");
        return;
    }
    let args = Args::parse();

    render::execute(args);
}

#[test]
fn full_test_run() {
    use std::path::Path;
    let color_file_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("color_files")
        .join("olsson.col")
        .to_str()
        .unwrap()
        .to_string();

    let args = Args {
        height: 1000,
        width: 1000,
        projection: "m".into(),
        longitude: -130.,
        latitude: 0.,
        magnification: 1.,
        color_filename: color_file_path,
        seed: 0.7609952,
        output_file: Some("./test_output".to_string()),
        draw_daylight: false,
        calculate_rainfall: false,
        latitude_color: 0,
        show_biomes: false,
        use_xpm_format: false,
        use_ppm_format: false,
        use_heightfield_format: false,
        use_png_format: false,
        use_bmp_format: true,
        map_rotation: vec![0., 0.],
        altitude_variation: 0.45,
        use_delta_map: None,
        distance_variation: 0.035,
        light_longitude: 0.,
        light_latitude: 0.,
        draw_land_edge: None,
        draw_outline_map: None,
        make_wrinkly_map: false,
        initial_altitude: -0.02,
        longitude_gridsize: 0.,
        latitude_gridsize: 0.,
        use_temperature: false,
        use_bumpmap: false,
        use_land_only_bumpmap: true,
        use_nonlinear_altitude_scaling: false,
        help: None,
        version: None,
        render_threads: 8,
    };

    render::execute(args);
}

#[derive(Parser)]
#[clap(disable_help_flag = true)]
#[clap(disable_version_flag = true)]
#[command(version)]
struct Args {
    /// Prints this help message
    #[arg(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,

    /// Print version info
    #[arg(short = 'R', required = false, action = clap::ArgAction::Version)]
    version: Option<bool>,

    /// Terrain generation seed
    #[arg(short = 's', value_name = "seed", default_value_t = 0.123)]
    seed: f64,

    /// Width in pixels
    #[arg(short = 'w', value_name = "width", default_value_t = 800)]
    width: u32,

    /// Height in pixels
    #[arg(short = 'h', value_name = "height", default_value_t = 600)]
    height: u32,

    /// Magnification level
    #[arg(short = 'm', value_name = "zoom", default_value_t = 1.0)]
    magnification: f64,

    /// Number of threads (1-255) used to render
    #[arg(long = "threads", value_name = "render-threads", default_value_t = 1)]
    render_threads: u8,

    /// Output file path. Outputs to standard output if missing
    #[arg(short = 'o', value_name = "filename", required = false)]
    output_file: Option<String>,

    /// Longitude of center, in degrees.
    #[arg(
        short = 'l',
        value_name = "width",
        allow_negative_numbers = true,
        default_value_t = 0.0
    )]
    longitude: f64,

    /// Latitude of center, in degrees.
    #[arg(
        short = 'L',
        value_name = "width",
        allow_negative_numbers = true,
        default_value_t = 0.0
    )]
    latitude: f64,

    /// Degrees between vertical gridlines.
    #[arg(short = 'g', value_name = "gridsize", default_value_t = 0.0)]
    latitude_gridsize: f64,

    /// Degrees between horizontal gridlines.
    #[arg(short = 'G', value_name = "gridsize", default_value_t = 0.0)]
    longitude_gridsize: f64,

    /// Initial land level altitude.
    #[arg(
        short = 'i', value_name = "altitude", allow_negative_numbers = true, default_value_t = -0.02
    )]
    initial_altitude: f64,

    /// Color depends on latitude. Repeats increase intensity.
    #[arg(short = 'c', action = clap::ArgAction::Count)]
    latitude_color: u8,

    /// Apply non-linear scaling to altitude. This makes land flatter near sea level.
    #[arg(short = 'n', default_value_t = false)]
    use_nonlinear_altitude_scaling: bool,

    /// Generate temperature map
    #[arg(short = 't', default_value_t = false)]
    use_temperature: bool,

    /// Calculate rainfall
    #[arg(short = 'r', default_value_t = false)]
    calculate_rainfall: bool,

    /// Make more “wrinkly” maps.
    #[arg(short = 'S', default_value_t = false)]
    make_wrinkly_map: bool,

    /// Read colour definitions from file.
    #[arg(short = 'C', value_name = "filename", default_value = "Olsson.col")]
    color_filename: String,

    /// Produce a black and white outline map. Ignores color file. Can optionally specify a contour line count to render.
    #[arg(
        short = 'O', value_name = "lines", num_args = 0..=1, required = false, default_missing_value = "0"
    )]
    draw_outline_map: Option<u8>,

    /// Outlines land in the color map's black value. Can optionally specify a contour line count to render.
    #[arg(
        short = 'E', value_name = "lines", num_args = 0..=1, required = false, default_missing_value = "0"
    )]
    draw_land_edge: Option<u8>,

    /// Use bumpmap shading. Land and water.
    #[arg(short = 'B', default_value_t = false)]
    use_bumpmap: bool,

    /// Use bumpmap shading. Land only.
    #[arg(short = 'b', default_value_t = false)]
    use_land_only_bumpmap: bool,

    /// Produces daylight shadows.
    #[arg(short = 'd', default_value_t = false)]
    draw_daylight: bool,

    /// Angle of “light” in bumpmap shading or longitude of sun in daylight shading.
    #[arg(short = 'a', value_name = "longitude", allow_negative_numbers = true, default_value_t = 150.0)]
    light_longitude: f64,

    /// Latitude of sun in daylight shading.
    #[arg(short = 'A', value_name = "latitude", allow_negative_numbers = true, default_value_t = 20.0)]
    light_latitude: f64,

    /// Output as PPM file format.
    #[arg(
        short = 'P', default_value_t = false
    )]
    use_ppm_format: bool,

    /// Output as XPM file format.
    #[arg(
        short = 'x', default_value_t = false
    )]
    use_xpm_format: bool,

    /// Output as PNG file format.
    #[arg(
        long = "png", default_value_t = false
    )]
    use_png_format: bool,

    /// Output as bitmap file format.
    #[arg(
        long = "bmp", default_value_t = false
    )]
    use_bmp_format: bool,

    /// Output as heightfield format.
    #[arg(short = 'H', default_value_t = false)]
    use_heightfield_format: bool,

    /// Read map from standard input and match new points to map if edge length greater than delta.
    #[arg(short = 'M', value_name = "delta", default_missing_value = "0.0")]
    use_delta_map: Option<f64>,

    /// Distance contribution to variation.
    #[arg(short = 'V', default_value_t = 0.035_f64)]
    distance_variation: f64,

    /// Altitude contribution to variation.
    #[arg(short = 'v', default_value_t = 0.45_f64)]
    altitude_variation: f64,

    /// Rotate map so what would otherwise be at latitude and longitude is moved to (0,0).
    /// This is different from using -l and -L because this rotation is done before applying
    /// gridlines and latitude-based effects.
    #[arg(
        short = 'T', num_args = 2, value_names = ["long", "lat"], allow_negative_numbers = true, default_values_t = [0.0, 0.0]
    )]
    map_rotation: Vec<f64>,

    /// Show biomes
    #[arg(short = 'z', default_value_t = false)]
    show_biomes: bool,

    /// Specifies projection:
    ///     m = Mercator
    ///     p : Peters
    ///     q : Square
    ///     s : Stereographic
    ///     o : Orthographic
    ///     g : Gnomonic
    ///     a : Area preserving azimuthal
    ///     c : Conical (conformal)
    ///     M : Mollweide
    ///     S : Sinusoidal (non-functional)
    ///     i : Icosahedral
    ///
    #[arg(
        short = 'p',
        value_name = "projection",
        default_value_t = String::from("m"),
        verbatim_doc_comment,
        value_parser = clap::builder::PossibleValuesParser::new(["m", "p", "q", "s", "o", "g", "a", "c", "M", "S", "i"]),
        hide_possible_values = true,
        hide_default_value = true
    )]
    projection: String,
}

pub fn get_commandline_footer() -> String {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        format!("Command line:\n{}\n", args[1..].join(" "))
    } else {
        "Command line:\n\n".into()
    }
}
