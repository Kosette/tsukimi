#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
mod utils;

use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;

#[derive(Default)]
struct TsukimiTool {
    config_path: Option<PathBuf>,
    convert_status_message: String,
    backup_status_message: String,
}

impl TsukimiTool {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (
                egui::TextStyle::Heading,
                egui::FontId::new(20.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(16.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();
        cc.egui_ctx.set_style(style);

        Default::default()
    }

    fn clear_state(&mut self) {
        self.config_path = None;
        self.backup_status_message = "".to_string();
        self.convert_status_message = "".to_string();
    }

    fn convert(&mut self) {
        if let Some(config) = &self.config_path {
            let result = utils::convert_toml_to_reg(config);

            match result {
                Ok(_) => self.convert_status_message = "Convert Success!".to_string(),
                Err(e) => self.convert_status_message = format!("Convert failed: {}", e),
            }
        }
    }

    fn backup(&mut self) {
        let result = utils::backup_registry();

        match result {
            Ok(_) => self.backup_status_message = "Backup Success!".to_string(),
            Err(e) => self.backup_status_message = format!("Backup failed: {}", e),
        }
    }
}

impl eframe::App for TsukimiTool {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tsukimi Tool");
            ui.add_space(5.0);
            ui.label("Convert and backup Tsukimi configs");
            ui.add_space(15.0);

            if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
                let dropped_files = ctx.input(|i| i.raw.dropped_files.clone());
                if let Some(config) = &dropped_files[1].path {
                    self.config_path = Some(config.to_path_buf());
                }
            }

            ui.separator();
            ui.add_space(10.0);
            ui.heading("Convert");
            ui.add_space(15.0);

            ui.horizontal(|ui| {
                if ui.button("Select Config File").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("config files", &["toml"])
                        .pick_file()
                    {
                        self.config_path = Some(path);
                    }
                }
            });

            if let Some(config) = &self.config_path {
                ui.label(format!("{}", config.display()));
            }

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let has_file = self.config_path.is_some();

                if ui
                    .add_enabled(has_file, egui::Button::new("Convert!"))
                    .clicked()
                {
                    self.convert();
                }

                if ui.button("Clear").clicked() {
                    self.clear_state();
                }
            });
            ui.label(self.convert_status_message.to_string());

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.heading("Backup");
            ui.add_space(15.0);

            ui.horizontal(|ui| {
                if ui.button("Backup!").clicked() {
                    self.backup();
                }

                if ui.button("Clear").clicked() {
                    self.clear_state();
                }
            });
            ui.label(self.backup_status_message.to_string());
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 420.0])
            .with_title("Tsukimi Tool"),
        ..Default::default()
    };

    eframe::run_native(
        "Tsukimi Tool",
        native_options,
        Box::new(|cc| Ok(Box::new(TsukimiTool::new(cc)))),
    )
}
