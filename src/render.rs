use crate::color::{build_color_data, ColorTable};
use crate::file::bitmap::validate_size;
use crate::file::{write_file, ColorMode, FileType};
use crate::geometry::Tetra;
use crate::terrain::LatLong;
use crate::util::Vec2D;
use crate::{projection, Args};
use chrono::Utc;
use std::f64::consts::PI;
use std::sync::{Arc, RwLock};
use std::thread;
use gridlines::GridLines;
use slicing::Slicing;
use crate::math::{RenderSeeds, SeedGenerator};
use crate::projection::ProjectionMode;

mod altitude;
pub mod color;
mod slicing;
mod gridlines;

#[derive(Clone)]
pub struct RenderOptions {
    pub seed_gen: SeedGenerator,
    pub seeds: RenderSeeds,
    pub slicing: Slicing,
    pub scale: f64,
    pub output_file: Option<String>,
    pub filetypes: Vec<FileType>,
    pub generate_heightfield: bool,
    pub center_point: LatLong,
    pub gridsize: LatLong,
    pub initial_altitude: f64,
    pub altitude_color: u8,
    pub use_nonlinear_altitude_scaling: bool,
    pub make_wrinkly_map: bool,
    pub color_filename: String,
    pub draw_outline_map: bool,
    pub draw_coastline: bool,
    pub land_contour_lines: u16,
    pub water_contour_lines: u16,
    pub light: LatLong,
    pub delta_map: Option<f64>,
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
}

impl RenderOptions {
    pub fn get_filetypes(args: &Args) -> Vec<FileType> {
        let mut retval = vec![];
        if args.use_ppm_format {
            retval.push(FileType::ppm);
        }
        if args.use_xpm_format {
            retval.push(FileType::xpm);
        }
        if args.use_png_format {
            retval.push(FileType::png);
        }
        if args.use_heightfield_format {
            retval.push(FileType::heightfield);
        }
        if args.use_bmp_format {
            retval.push(FileType::bmp);
        }
        retval
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

        let seed_gen = SeedGenerator::new(&self.precision);
        
        RenderOptions {
            seeds: seed_gen.generate(self.seed),
            seed_gen,            
            slicing: Slicing::new(self.height, self.width, self.render_threads),
            scale: self.magnification.clamp(0.1, 100_000.0),
            output_file: self.output_file.clone(),
            filetypes: RenderOptions::get_filetypes(&self),
            generate_heightfield: self.use_heightfield_format,
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
            draw_coastline: self.draw_coastline,
            land_contour_lines: self.land_contour_lines,
            water_contour_lines: self.water_contour_lines,
            light: LatLong::new(self.light_latitude, self.light_longitude),
            delta_map: self.use_delta_map,
            map_rotation: LatLong::new(
                -self.map_rotation[0].to_radians(),
                -self.map_rotation[1].to_radians(),
            ),
            show_biomes: self.show_biomes,
            projection: match self.projection.as_str() {
                "m" => {
                    if self.latitude.to_radians().abs() >= PI - 1E-10 {
                        ProjectionMode::Stereographic
                    } else {
                        ProjectionMode::Mercator
                    }
                }
                "p" => ProjectionMode::Peters,
                "q" => ProjectionMode::Square,
                "s" => ProjectionMode::Stereographic,
                "o" => ProjectionMode::Orthographic,
                "g" => ProjectionMode::Gnomonic,
                "a" => ProjectionMode::Azimuthal,
                "c" => {
                    if self.latitude == 0. {
                        ProjectionMode::Mercator
                    } else if self.latitude.abs() >= 90. {
                        ProjectionMode::Stereographic
                    } else {
                        ProjectionMode::Conical
                    }
                }
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
            alt_diff_weight: self.altitude_variation
                / if self.make_wrinkly_map { 2.0 } else { 1.0 },
            alt_diff_power: if self.make_wrinkly_map { 0.75 } else { 1.0 },
            distance_weight: self.distance_variation,
            distance_power: 0.47,
        }
    }
}

pub struct RenderState {
    pub options: RenderOptions,
    pub canvas: RwLock<Vec<Vec2D<u16>>>,
    pub heightfield: RwLock<Vec<Vec2D<i32>>>,
    pub shading: RwLock<Vec<Vec2D<u8>>>,
    pub color_table: ColorTable,
    pub search_map: [[i32; 30]; 60],
    pub grid_lines: GridLines,
    pub map_match_size: f64,
}

impl RenderState {
    pub fn new(options: RenderOptions) -> Self {
        Self {
            options: options.clone(),
            canvas: RwLock::new(vec![vec![]; options.slicing.slice_count as usize]),
            heightfield: RwLock::new(if options.generate_heightfield {
                vec![vec![]; options.slicing.slice_count as usize]
            } else {
                vec![]
            }),
            shading: RwLock::new(if options.shading_level > 0 {
                vec![vec![]; options.slicing.slice_count as usize]
            } else {
                vec![]
            }),
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
    pub options: RenderOptions,
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
    pub shade: u8,
}

impl ThreadState {
    pub fn new(id: u8, options: RenderOptions) -> Self {
        Self {
            options: options.clone(),
            local_height: options.slicing.get_slice_height(id),
            canvas: gen_canvas(id, &options),
            heightfield: gen_heightfield(id, &options),
            shading: gen_shading(id, &options),
            color_table: build_color_data(&options),
            search_map: [[0; 30]; 60],
            base_tetra: crate::geometry::create_base_tetra(&options),
            cached_tetra: crate::geometry::create_base_tetra(&options),
            starting_subdivision_depth: 0,
            rain_shadow: 0.0,
            shade: 0,
        }
    }
}

fn gen_canvas(id: u8, options: &RenderOptions) -> Vec2D<u16> {
    if !options.filetypes.contains(&FileType::heightfield) || options.filetypes.len() > 1 {
        vec![vec![0; options.slicing.width]; options.slicing.get_slice_height(id)]
    } else {
        vec![]
    }
}

fn gen_heightfield(id: u8, options: &RenderOptions) -> Vec2D<i32> {
    if options.generate_heightfield {
        vec![vec![0; options.slicing.width]; options.slicing.get_slice_height(id)]
    } else {
        vec![]
    }
}

fn gen_shading(id: u8, options: &RenderOptions) -> Vec2D<u8> {
    if (!options.filetypes.contains(&FileType::heightfield) || options.filetypes.len() > 1)
        && options.shading_level > 0
    {
        vec![vec![0; options.slicing.width]; options.slicing.get_slice_height(id)]
    } else {
        vec![]
    }
}

pub fn execute(args: Args) {
    let options = args.into_options();
    let state = Arc::new(RenderState::new(options.clone()));

    validate_size(state.clone());
    let now = Utc::now();

    thread::scope(|scope| {
        for thread_id in 0..options.slicing.slice_count {
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
                    ProjectionMode::Peters => projection::peters::render(thread_id, state.clone()),
                    ProjectionMode::Sinusoidal => {
                        projection::sinusoidal::render(thread_id, state.clone())
                    }
                    ProjectionMode::Square => projection::square::render(thread_id, state.clone()),
                    ProjectionMode::Stereographic => {
                        projection::stereographic::render(thread_id, state.clone())
                    }
                };
                println!("Ending thread {thread_id}");
            });
        }
    });

    gridlines::generate_gridlines(state.clone());

    smooth_shading(state.clone());

    let _ = write_file(state.clone());
    let time = (Utc::now() - now).num_seconds();
    println!("Render completed in {time} seconds");
}

pub fn commit_render_data(
    thread_id: u8,
    thread_state: ThreadState,
    render_state: Arc<RenderState>,
) {
    if thread_state
        .options
        .filetypes
        .contains(&FileType::heightfield)
    {
        render_state.heightfield.write().unwrap()[thread_id as usize] = thread_state.heightfield;
    }

    if !thread_state
        .options
        .filetypes
        .contains(&FileType::heightfield)
        || thread_state.options.filetypes.len() > 1
    {
        render_state.canvas.write().unwrap()[thread_id as usize] = thread_state.canvas;
        if thread_state.options.shading_level > 0 {
            render_state.shading.write().unwrap()[thread_id as usize] = thread_state.shading;
        }
    }
}

// TODO: Generate contour lines and outlines
fn generate_outlines(state: Arc<RenderState>) {
    let sea_bottom = state.color_table.sea_bottom;
    let sea_level = state.color_table.sea_level;
    let lowest_land = state.color_table.lowest_land;
    let highest_land = state.color_table.highest_land;

    let land_contour_step = (highest_land - lowest_land) / (state.options.land_contour_lines + 1);
    let water_contour_step = (lowest_land - sea_bottom) / 20;

    let canvas = state.canvas.write().unwrap();

    // for h in 1..state.options.height as usize {
    //     for w in 1..state.options.width as usize {
    //         //let point = canvas
    //         // detect line for beaches
    // 
    //         if state.options.land_contour_lines > 0 {
    //             // detect land lines
    //         }
    // 
    //         if state.options.water_contour_lines > 0 {
    //             // detect water lines
    //         }
    // 
    //         if state.options.draw_outline_map {}
    //     }
    // }
    // 
    // for h in 0..state.options.height as usize {
    //     for w in 0..state.options.width as usize {
    //         // wipe colors
    //     }
    // }

    // Apply outlines to canvas
}

// TODO: Eventually add in map reading (from file or stdin)
fn read_map() {}

fn smooth_shading(state: Arc<RenderState>) {
    let mut shading = state.shading.write().unwrap();
    if shading.len() == 0 {
        return;
    }
    let height_limit = state.options.slicing.height - 1;
    let width_limit = state.options.slicing.width - 1;

    for ahi in 0..height_limit {
        let (vi, hi) = state.options.slicing.translate_index(ahi);
        for wi in 0..width_limit {
            let p = 4 * shading[vi][hi][wi] as u16;
            let w1 = 2 * shading[vi][hi][wi + 1] as u16;
            let (vi1, hi1) = state.options.slicing.translate_index(ahi + 1);
            let h1 =  2 * shading[vi1][hi1][wi] as u16;
            let hw1 = shading[vi1][hi1][wi + 1] as u16;
            shading[vi][hi][wi] = ((p + h1 + w1 + hw1 + 4) / 9).min(255) as u8;
        }
    }
}

