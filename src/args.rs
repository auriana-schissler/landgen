use clap::Parser;
use std::env;

#[derive(Clone, Parser)]
#[clap(disable_help_flag = true)]
#[clap(disable_version_flag = true)]
#[command(version)]
pub struct Args {
    /// Prints this help message
    #[arg(long, action = clap::ArgAction::HelpLong)]
    pub help: Option<bool>,

    /// Print version info
    #[arg(short = 'R', required = false, action = clap::ArgAction::Version)]
    pub version: Option<bool>,

    /// Terrain generation seed
    #[arg(short = 's', value_name = "seed", default_value_t = Args::default().seed)]
    pub seed: f64,

    /// Width in pixels
    #[arg(short = 'w', value_name = "width", default_value_t = Args::default().width)]
    pub width: usize,

    /// Height in pixels
    #[arg(short = 'h', value_name = "height", default_value_t = Args::default().height)]
    pub height: usize,

    /// Magnification level
    #[arg(short = 'm', value_name = "zoom", default_value_t = Args::default().magnification)]
    pub magnification: f64,

    /// Number of threads (1-255) used to render
    #[arg(long = "threads", value_name = "render-threads", default_value_t = Args::default().render_threads)]
    pub render_threads: u8,

    /// Output file path. Outputs to standard output if missing
    #[arg(short = 'o', value_name = "filename", required = false)]
    pub output_file: Option<String>,

    /// Longitude of center, in degrees.
    #[arg(
        short = 'l',
        value_name = "width",
        allow_negative_numbers = true,
        default_value_t = Args::default().longitude
    )]
    pub longitude: f64,

    /// Latitude of center, in degrees.
    #[arg(
        short = 'L',
        value_name = "width",
        allow_negative_numbers = true,
        default_value_t = Args::default().latitude
    )]
    pub latitude: f64,

    /// Degrees between vertical gridlines.
    #[arg(short = 'g', value_name = "grid_size", default_value_t = Args::default().latitude_grid_size)]
    pub latitude_grid_size: f64,

    /// Degrees between horizontal gridlines.
    #[arg(short = 'G', value_name = "grid_size", default_value_t = Args::default().longitude_grid_size)]
    pub longitude_grid_size: f64,

    /// Initial land level altitude.
    #[arg(
        short = 'i', value_name = "altitude", allow_negative_numbers = true, default_value_t = Args::default().initial_altitude
    )]
    pub initial_altitude: f64,

    /// Color depends on latitude. Repeats increase intensity.
    #[arg(short = 'c', action = clap::ArgAction::Count)]
    pub latitude_color: u8,

    /// Apply non-linear scaling to altitude. This makes land flatter near sea level.
    #[arg(short = 'n', default_value_t = Args::default().use_nonlinear_altitude_scaling)]
    pub use_nonlinear_altitude_scaling: bool,

    /// Generate temperature map
    #[arg(short = 't', default_value_t = Args::default().use_temperature)]
    pub use_temperature: bool,

    /// Calculate rainfall
    #[arg(short = 'r', default_value_t = Args::default().calculate_rainfall)]
    pub calculate_rainfall: bool,

    /// Make more “wrinkly” maps.
    #[arg(short = 'S', default_value_t = Args::default().make_wrinkly_map)]
    pub make_wrinkly_map: bool,

    /// Read color definitions from file.
    #[arg(short = 'C', value_name = "filename", default_value = "")]
    pub color_filename: String,

    /// Read color definitions from file.
    #[arg(short = 'C', value_name = "filename", default_value = "Olsson")]
    pub color_pallet_name: String,

    /// Ignores all colors but black(0) and white(1) on the color file.
    #[arg(short = 'O', requires = "draw_coastline", default_value_t = Args::default().draw_outline_map)]
    pub draw_outline_map: bool,

    /// Draws coastlines in the color map's black value.
    #[arg(short = 'E', default_value_t = Args::default().draw_coastline)]
    pub draw_coastline: bool,

    /// Draws a number of contour lines on land
    #[arg(long = "land-lines", value_name = "land-lines", default_value_t = Args::default().land_contour_lines)]
    pub land_contour_lines: u16,

    /// Draws a number of contour lines on water
    #[arg(long = "water-lines", value_name = "water-lines", default_value_t = Args::default().water_contour_lines)]
    pub water_contour_lines: u16,

    /// Use bump map shading. Land and water.
    #[arg(short = 'B', default_value_t = Args::default().use_bump_map)]
    pub use_bump_map: bool,

    /// Use bump map shading. Land only.
    #[arg(short = 'b', default_value_t = Args::default().use_land_only_bump_map)]
    pub use_land_only_bump_map: bool,

    /// Produces daylight shadows.
    #[arg(short = 'd', default_value_t = Args::default().draw_daylight)]
    pub draw_daylight: bool,

    /// Angle of “light” in bump map shading or longitude of sun in daylight shading.
    #[arg(
        short = 'a',
        value_name = "longitude",
        allow_negative_numbers = true,
        default_value_t = Args::default().light_longitude
    )]
    pub light_longitude: f64,

    /// Latitude of sun in daylight shading.
    #[arg(
        short = 'A',
        value_name = "latitude",
        allow_negative_numbers = true,
        default_value_t = Args::default().light_latitude
    )]
    pub light_latitude: f64,

    /// Output as PPM file format.
    #[arg(short = 'P', default_value_t = Args::default().use_ppm_format)]
    pub use_ppm_format: bool,

    /// Output as XPM file format.
    #[arg(short = 'x', default_value_t = Args::default().use_xpm_format)]
    pub use_xpm_format: bool,

    /// Output as PNG file format.
    #[arg(long = "png", default_value_t = Args::default().use_png_format)]
    pub use_png_format: bool,

    /// Output as bitmap file format.
    #[arg(long = "bmp", default_value_t = Args::default().use_bmp_format)]
    pub use_bmp_format: bool,

    /// Output as heightfield format.
    #[arg(short = 'H', default_value_t = Args::default().use_heightfield_format)]
    pub use_heightfield_format: bool,

    /// Read map from standard input and match new points to map if edge length greater than delta.
    #[arg(short = 'M', value_name = "delta", default_missing_value = "0.0")]
    pub use_delta_map: Option<f64>,

    /// Distance contribution to variation.
    #[arg(short = 'V', default_value_t = Args::default().distance_variation)]
    pub distance_variation: f64,

    /// Altitude contribution to variation.
    #[arg(short = 'v', default_value_t = Args::default().altitude_variation)]
    pub altitude_variation: f64,

    /// Rotate map so what would otherwise be at latitude and longitude is moved to (0,0).
    /// This is different from using -l and -L because this rotation is done before applying
    /// gridlines and latitude-based effects.
    #[arg(
        short = 'T', num_args = 2, value_names = ["long", "lat"], allow_negative_numbers = true, default_values_t = Args::default().map_rotation
    )]
    pub map_rotation: Vec<f64>,

    /// Show biomes
    #[arg(short = 'z', default_value_t = Args::default().show_biomes)]
    pub show_biomes: bool,

    /// Specify the randomness precision with Original (o), Normal (n), or High (h).
    #[arg(long = "precision", default_value_t = Args::default().precision)]
    pub precision: String,

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
    pub projection: String,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            help: None,
            version: None,
            seed: 0.123,
            width: 800,
            height: 600,
            magnification: 1.0,
            render_threads: 1,
            output_file: None,
            longitude: 0.0,
            latitude: 0.0,
            latitude_grid_size: 0.0,
            longitude_grid_size: 0.0,
            initial_altitude: -0.02,
            latitude_color: 0,
            use_nonlinear_altitude_scaling: false,
            use_temperature: false,
            calculate_rainfall: false,
            make_wrinkly_map: false,
            color_filename: "".into(),
            color_pallet_name: "olsson".into(),
            draw_outline_map: false,
            draw_coastline: false,
            land_contour_lines: 0,
            water_contour_lines: 0,
            use_bump_map: false,
            use_land_only_bump_map: false,
            draw_daylight: false,
            light_longitude: 150.0,
            light_latitude: 20.0,
            use_ppm_format: false,
            use_xpm_format: false,
            use_png_format: false,
            use_bmp_format: false,
            use_heightfield_format: false,
            use_delta_map: None,
            distance_variation: 0.035,
            altitude_variation: 0.45,
            map_rotation: vec![0.0, 0.0],
            show_biomes: false,
            precision: "oooo".into(),
            projection: "m".into(),
        }
    }
}

pub fn get_commandline_footer() -> String {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        format!("Command line:{}", args[1..].join(" "))
    } else {
        "Command line:".into()
    }
}
