use std::f64::consts::PI;
use crate::color::{build_color_data, ColorTable};
use crate::file::bitmap::validate_size;
use crate::file::{write_file, ColorMode, FileType};
use crate::geometry::{Tetra, Vertex};
use crate::math::rand_low;
use crate::terrain::LatLong;
use crate::util::Vec2D;
use crate::{projection, Args};
use chrono::Utc;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread;

mod altitude;
pub mod color;

#[derive(Clone)]
pub struct RenderOptions {
    pub seed: f64,
    pub width: i32,
    pub height: i32,
    pub scale: f64,
    pub output_file: Option<String>,
    pub filetype: FileType,
    pub center_point: LatLong,
    pub gridsize: LatLong,
    pub initial_altitude: f64,
    pub altitude_color: u8,
    pub use_nonlinear_altitude_scaling: bool,
    pub make_wrinkly_map: bool,
    pub color_filename: String,
    pub draw_outline_map: Option<u8>,
    pub draw_land_edge: Option<u8>,
    pub daylight: LatLong,
    pub delta_map: Option<f64>,
    pub distance_variation: bool,
    pub altitude_variation: bool,
    pub map_rotation: LatLong,
    pub show_biomes: bool,
    pub projection: ProjectionMode,
    pub use_temperature: bool,
    pub calculate_rainfall: bool,
    pub use_latitude_coloring: bool,
    pub latitude_color_intensity: u8,
    pub shading_level: u8,
    pub alt_diff_weight: f64,
    pub alt_diff_power: f64,
    pub distance_weight: f64,
    pub distance_power: f64,
    pub render_threads: usize,
}

impl RenderOptions {
    pub fn get_filetype(args: &Args) -> FileType {
        if args.use_ppm_format {
            FileType::ppm
        } else if args.use_xpm_format {
            FileType::xpm
        } else if args.use_png_format {
            FileType::png
        } else if args.use_heightfield_format {
            FileType::heightfield
        } else {
            FileType::bmp
        }
    }
}

impl Args {
    pub fn into_options(mut self) -> RenderOptions {
        self.latitude = self.latitude.clamp(-90.0, 90.0);
        while self.longitude < -180.0 {
            self.longitude += 360.0;
        }
        while self.longitude > 180.0 {
            self.longitude -= 360.0;
        }
        while self.map_rotation[0] < -180.0 {
            self.map_rotation[0] += 360.0;
        }
        while self.map_rotation[0] > 180.0 {
            self.map_rotation[0] -= 360.0;
        }
        while self.map_rotation[1] < -180.0 {
            self.map_rotation[1] += 360.0;
        }
        while self.map_rotation[1] > 180.0 {
            self.map_rotation[1] -= 360.0;
        }

        RenderOptions {
            seed: self.seed,
            width: self.width as i32,
            height: self.height as i32,
            scale: self.magnification.clamp(0.1, 100_000.0),
            output_file: self.output_file.clone(),
            filetype: RenderOptions::get_filetype(&self),
            center_point: LatLong::new_with_trig(
                self.latitude.to_radians(),
                self.longitude.to_radians(),
            ),
            gridsize: LatLong::new(self.latitude_gridsize, self.longitude_gridsize),
            initial_altitude: self.initial_altitude,
            altitude_color: self.latitude_color,
            use_nonlinear_altitude_scaling: self.use_nonlinear_altitude_scaling,
            make_wrinkly_map: self.make_wrinkly_map,
            color_filename: self.color_filename.clone(),
            draw_outline_map: self.draw_outline_map,
            draw_land_edge: self.draw_land_edge,
            daylight: LatLong::new(self.light_latitude, self.light_longitude),
            delta_map: self.use_delta_map,
            distance_variation: self.distance_variation,
            altitude_variation: self.altitude_variation,
            map_rotation: LatLong::new(
                -self.map_rotation[0].to_radians(),
                -self.map_rotation[1].to_radians(),
            ),
            show_biomes: self.show_biomes,
            projection: match self.projection.as_str() {
                "m" => {
                    if self.latitude.abs() >= PI - 1E-10 {
                        ProjectionMode::Stereographic
                    } else {
                        ProjectionMode::Mercator
                    }
                },
                "p" => ProjectionMode::Peters,
                "q" => ProjectionMode::Square,
                "s" => ProjectionMode::Stereographic,
                "o" => ProjectionMode::Orthographic,
                "g" => ProjectionMode::Gnomonic,
                "a" => ProjectionMode::Azimuthal,
                "c" => {
                    if self.latitude == 0. {
                        ProjectionMode::Mercator
                    }
                    else if self.latitude.abs() >= 90. {
                        ProjectionMode::Stereographic
                    } else {
                        ProjectionMode::Conical
                    }
                },
                "M" => ProjectionMode::Mollweide,
                "S" => ProjectionMode::Sinusoidal,
                "i" => ProjectionMode::Icosahedral,
                _ => panic!(""),
            },
            use_temperature: self.use_temperature,
            calculate_rainfall: self.calculate_rainfall,
            use_latitude_coloring: self.latitude_color > 0,
            latitude_color_intensity: self.latitude_color,
            shading_level: if self.draw_daylight {
                3
            } else if self.use_land_only_bumpmap {
                2
            } else if self.use_bumpmap {
                1
            } else {
                0
            },
            alt_diff_weight: 0.45,
            alt_diff_power: 1.0,
            distance_weight: 0.035,
            distance_power: 0.47,
            render_threads: self.render_threads as usize,
        }
    }
}

pub struct GridLines {
    pub x: Vec2D<f64>,
    pub y: Vec2D<f64>,
    pub z: Vec2D<f64>,
}

impl GridLines {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            x: vec![vec! {0.0; width as usize}; height as usize],
            y: vec![vec! {0.0; width as usize}; height as usize],
            z: vec![vec! {0.0; width as usize}; height as usize],
        }
    }
}

fn create_base_tetra(options: Rc<RenderOptions>) -> Tetra {
    let seeds = RenderSeeds::new(options.seed);
    Tetra {
        a: Vertex {
            x: -3.0_f64.sqrt() - 0.20,
            y: -3.0_f64.sqrt() - 0.22,
            z: -3.0_f64.sqrt() - 0.23,
            seed: seeds.ss1,
            altitude: options.initial_altitude,
            rain_shadow: 0.,
        },
        b: Vertex {
            x: -3.0_f64.sqrt() - 0.19,
            y: 3.0_f64.sqrt() + 0.18,
            z: 3.0_f64.sqrt() + 0.17,
            seed: seeds.ss2,
            altitude: options.initial_altitude,
            rain_shadow: 0.,
        },
        c: Vertex {
            x: 3.0_f64.sqrt() + 0.21,
            y: -3.0_f64.sqrt() - 0.24,
            z: 3.0_f64.sqrt() + 0.15,
            seed: seeds.ss3,
            altitude: options.initial_altitude,
            rain_shadow: 0.,
        },
        d: Vertex {
            x: 3.0_f64.sqrt() + 0.24,
            y: 3.0_f64.sqrt() + 0.22,
            z: -3.0_f64.sqrt() - 0.25,
            seed: seeds.ss4,
            altitude: options.initial_altitude,
            rain_shadow: 0.,
        },
    }
}

pub struct RenderState {
    pub options: RenderOptions,
    pub canvas: RwLock<Vec<Vec2D<u16>>>,
    pub heightfield: Vec<Vec2D<i32>>,
    pub shading: Vec<Vec2D<u8>>,
    pub color_table: ColorTable,
    pub search_map: [[i32; 30]; 60],
    pub grid_lines: GridLines,
    pub map_match_size: f64,
}

impl RenderState {
    pub fn new(options: RenderOptions) -> Self {
        Self {
            options: options.clone(),
            canvas: RwLock::new(vec![vec![]; options.render_threads]),
            heightfield: Vec::with_capacity(options.render_threads),
            shading: Vec::with_capacity(options.render_threads),
            color_table: build_color_data(&options),
            search_map: [[0; 30]; 60],
            grid_lines: GridLines::new(0, 0),
            map_match_size: 0.0,
        }
    }

    pub fn get_color_mode(&self) -> ColorMode {
        if self.color_table.is_monochrome() {
            ColorMode::Monochrome
        } else {
            ColorMode::Color
        }
    }
}

pub struct ThreadState {
    pub options: Rc<RenderOptions>,
    pub starting_line: usize,
    pub local_height: usize,
    pub canvas: Vec2D<u16>,
    pub heightfield: Vec2D<i32>,
    pub shading: Vec2D<u8>,
    pub color_table: ColorTable,
    pub search_map: [[i32; 30]; 60],
    pub base_tetra: Tetra,
    pub cached_tetra: Tetra,
    pub starting_subdivision_depth: u8,
    pub rain_shadow: f64,
    pub shade: i32,
}

impl ThreadState {
    pub fn new(id: usize, options: Rc<RenderOptions>) -> Self {
        let height_segment_size =
            (options.height as f64 / options.render_threads as f64).ceil() as usize;
        let starting_line = id * height_segment_size;
        let local_height = height_segment_size.min(options.height as usize - starting_line);
        let starting_subdivision_depth = 0;
        Self {
            options: options.clone(),
            starting_line,
            local_height,
            canvas: vec![vec![0; options.width as usize]; local_height],
            heightfield: gen_heightfield(id, options.clone()),
            shading: gen_shading(id, options.clone()),
            color_table: build_color_data(&options),
            search_map: [[0; 30]; 60],
            base_tetra: create_base_tetra(options.clone()),
            cached_tetra: create_base_tetra(options.clone()),
            starting_subdivision_depth,
            rain_shadow: 0.0,
            shade: 0,
        }
    }
}

fn gen_heightfield(id: usize, options: Rc<RenderOptions>) -> Vec2D<i32> {
    let height_segment_size =
        (options.height as f64 / options.render_threads as f64).ceil() as usize;
    let starting_line = id * height_segment_size;
    let local_height = height_segment_size.min(options.height as usize - starting_line);
    match options.filetype {
        FileType::heightfield => vec![vec![0; options.width as usize]; local_height],
        _ => vec![],
    }
}

fn gen_shading(id: usize, options: Rc<RenderOptions>) -> Vec2D<u8> {
    let height_segment_size =
        (options.height as f64 / options.render_threads as f64).ceil() as usize;
    let starting_line = id * height_segment_size;
    let local_height = height_segment_size.min(options.height as usize - starting_line);
    match options.filetype {
        FileType::heightfield => vec![vec![0; options.width as usize]; local_height],
        _ => vec![],
    }
}

#[derive(Clone)]
struct RenderSeeds {
    pub base: f64,
    pub ss1: f64,
    pub ss2: f64,
    pub ss3: f64,
    pub ss4: f64,
}

impl RenderSeeds {
    pub fn new(seed: f64) -> Self {
        let ss1 = rand_low(seed, seed);
        let ss2 = rand_low(ss1, ss1);
        let ss3 = rand_low(ss1, ss2);
        let ss4 = rand_low(ss2, ss3);

        Self {
            base: seed,
            ss1,
            ss2,
            ss3,
            ss4,
        }
    }
}

pub fn execute(args: Args) {
    let options = args.into_options();
    let state = Arc::new(RenderState::new(options.clone()));

    validate_size(state.clone());
    println!("Starting render");
    let now = Utc::now();

    thread::scope(|scope| {
        for thread_id in 0..options.render_threads {
            println!("Spawning thread {thread_id}");
            let state = state.clone();
            scope.spawn(move || {
                match state.options.projection {
                    ProjectionMode::Azimuthal => {
                        projection::azimuthal::render(thread_id, state.clone())
                    }
                    ProjectionMode::Conical => {
                        projection::conical::render(thread_id, state.clone())
                    }
                    ProjectionMode::Gnomonic => {
                        projection::gnomonic::render(thread_id, state.clone())
                    }
                    ProjectionMode::Icosahedral => {
                        projection::icosahedral::render(thread_id, state.clone())
                    }
                    ProjectionMode::Mercator => {
                        projection::mercator::render(thread_id, state.clone())
                    }
                    ProjectionMode::Mollweide => {
                        projection::mollweide::render(thread_id, state.clone())
                    }
                    ProjectionMode::Orthographic => {
                        projection::orthographic::render(thread_id, state.clone())
                    }
                    ProjectionMode::Peters => {
                        projection::peters::render(thread_id, state.clone())
                    }
                    _ => panic!(),
                };
                println!("Ending thread {thread_id}");
            });
        }
    });

    // make grid lines

    // smooth shading

    let _ = write_file(state.clone());
    let time = (Utc::now() - now).num_seconds();
    println!("Render completed in {time} seconds");
}

fn make_outline() {}

fn read_map() {}

fn smooth_shading() {}

#[derive(Clone)]
pub enum ProjectionMode {
    Mercator,
    Peters,
    Square,
    Stereographic,
    Orthographic,
    Gnomonic,
    Azimuthal,
    Conical,
    Mollweide,
    Sinusoidal,
    Icosahedral,
}
