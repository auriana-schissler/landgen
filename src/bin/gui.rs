#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::Widget;
use landgen::args::Args;
use landgen::file::write_to_file;
use landgen::render::render_map;

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

#[derive(Default)]
struct GuiState {
    args: Args,
    seed_value: String
}

impl eframe::App for GuiState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Seed:");
                egui::TextEdit::singleline(&mut self.seed_value)
                    .char_limit(12)
                    .desired_width(100.0)
                    .ui(ui);
                if ui.button("Render").clicked() {
                    let args = create_local_args(&self.args);
                    let state = render_map(args);
                    write_to_file(state.clone()).unwrap();
                    ui.ctx().forget_image("file://./preview.png");
                }
            });
            ui.image("file://./preview.png");
            
            //
            // ui.heading("landgen");
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Increment").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));
            //
            // ui.image(egui::include_image!("../../ferris.png"));
        });
    }
}

fn create_local_args(original_args: &Args) -> Args {
    Args {
        height: 300,
        width: 300,
        render_threads: 8,
        output_file: Some("preview".into()),
        use_bmp_format: false,
        use_heightfield_format: false,
        use_png_format: true,
        use_ppm_format: false,
        use_xpm_format: false,
        ..original_args.clone()
    }
}
