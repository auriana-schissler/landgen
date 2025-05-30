#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::egui::load::Bytes;
use eframe::egui;
use eframe::egui::{ImageSource, ViewportId, Widget};
use eframe::epaint::TextureManager;
use landgen::args::Args;
use landgen::file::{write_out, write_to_file};
use landgen::render::render_map;
use rand::random_range;
use std::borrow::Cow;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "landgen",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<GuiState>::default())
        }),
    )
}

struct GuiState {
    args: Args,
    seed_value: String,
    preview_height: String,
    preview_width: String,
    height: String,
    width: String,
    current_image: Arc<[u8]>,
    current_image_name: String,
    new_image: Arc<[u8]>,
    new_image_name: String,
    latitude: f64,
    longitude: f64,
    zoom: u32,
}

impl Default for GuiState {
    fn default() -> Self {
        let args = Args::default();
        GuiState {
            seed_value: args.seed.to_string(),
            args,
            preview_height: "500".into(),
            preview_width: "500".into(),
            height: "500".into(),
            width: "500".into(),
            current_image: Default::default(),
            current_image_name: Default::default(),
            new_image: Default::default(),
            new_image_name: Default::default(),
            latitude: 0.0,
            longitude: 0.0,
            zoom: 100
        }
    }
}

impl eframe::App for GuiState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.height.parse::<u32>().is_err() {
            self.height = self
                .height
                .chars()
                .filter(|&c| c.is_numeric())
                .collect::<String>();
        }
        if self.width.parse::<u32>().is_err() {
            self.width = self
                .width
                .chars()
                .filter(|&c| c.is_numeric())
                .collect::<String>();
        }

        ctx.show_viewport_immediate(
            ViewportId::from_hash_of("options_panel"),
            egui::ViewportBuilder::default()
                .with_title("Options")
                .with_inner_size([300.0, 500.0])
                .with_close_button(false)
                .with_maximize_button(false),
            |ctx, _| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label("Options");

                        ui.horizontal(|ui| {
                            ui.label("Seed");
                            egui::TextEdit::singleline(&mut self.seed_value)
                                .char_limit(12)
                                .desired_width(100.0)
                                .ui(ui);
                        });

                        if ui.button("Randomize Seed & Preview").clicked() {
                            let new_seed = random_range(0_f64..1.0);
                            self.seed_value = format!("{:.8}", new_seed);
                            preview(self);
                        }

                        ui.horizontal(|ui| {
                            ui.label("Preview Height");
                            egui::TextEdit::singleline(&mut self.preview_height)
                                .desired_width(100.0)
                                .interactive(false)
                                .ui(ui);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Preview Width");
                            egui::TextEdit::singleline(&mut self.preview_width)
                                .desired_width(100.0)
                                .interactive(false)
                                .ui(ui);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Render Height");
                            egui::TextEdit::singleline(&mut self.height)
                                .char_limit(6)
                                .desired_width(100.0)
                                .interactive(false)
                                .ui(ui);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Render Width");
                            egui::TextEdit::singleline(&mut self.width)
                                .char_limit(6)
                                .desired_width(100.0)
                                .interactive(false)
                                .ui(ui);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Latitude Offset");
                            egui::DragValue::new(&mut self.latitude)
                                .range(-90.0..=90.0)
                                .fixed_decimals(2)
                                .speed(0.1)
                                .suffix('°')
                                .ui(ui);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Longitude Offset");
                            egui::DragValue::new(&mut self.longitude)
                                .range(-180.0..=180.0)
                                .fixed_decimals(2)
                                .speed(0.1)
                                .suffix('°')
                                .ui(ui);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Zoom");
                            egui::DragValue::new(&mut self.zoom)
                                .range(20..=100000)
                                .speed(1)
                                .suffix('%')
                                .ui(ui);
                        });

                        if ui.button("Preview").clicked() {
                            preview(self);
                        }

                        if ui.button("Render").clicked() {
                            render(self);
                        }
                    });
                });
            },
        );

        egui::CentralPanel::default().show(ctx, |ui| {
            self.preview_height = (ui.available_size().y.floor() as usize).to_string();
            self.preview_width = (ui.available_size().x.floor() as usize).to_string();

            let old_image_name = if !self.new_image.is_empty() {
                self.current_image = self.new_image.clone();
                ui.ctx().forget_image(&self.current_image_name);
                let old_image_name = self.current_image_name.clone();
                self.current_image_name = self.new_image_name.clone();
                self.new_image = Arc::new([]);
                self.new_image_name = "".into();
                old_image_name
            } else {
                Default::default()
            };
            if self.current_image.is_empty() {
                preview(self);
            }
            ui.image(ImageSource::Bytes {
                uri: Cow::Owned(self.current_image_name.clone()),
                bytes: Bytes::Shared(self.current_image.clone()),
            });
            if !old_image_name.is_empty() {
                ui.ctx().forget_image(&old_image_name);
            }
        });
    }
}

fn preview(state: &mut GuiState) {
    let args = create_local_args(state);
    let render_state = render_map(&args);

    let mut image_data = vec![];
    write_out(render_state.clone(), &mut image_data).unwrap();

    state.new_image = Arc::from(image_data);
    state.new_image_name = format!(
        "bytes://{}{}",
        state.seed_value,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
}

fn render(state: &mut GuiState) {
    let args = create_local_args(state);
    let render_state = render_map(&args);

    write_to_file(render_state.clone()).unwrap();
}

fn create_local_args(state: &GuiState) -> Args {
    Args {
        seed: state.seed_value.parse::<f64>().unwrap(),
        height: state.preview_height.parse().unwrap(),
        width: state.preview_width.parse().unwrap(),
        latitude: state.latitude,
        longitude: state.longitude,
        magnification: state.zoom as f64 / 100.,
        render_threads: 8,
        output_file: Some("preview".into()),
        use_bmp_format: false,
        use_heightfield_format: false,
        use_png_format: true,
        use_ppm_format: false,
        use_xpm_format: false,
        ..state.args.clone()
    }
}
