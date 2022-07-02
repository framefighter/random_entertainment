use egui::{Color32, RichText};
use egui_extras::RetainedImage;

use self::api::ResponseStreamsData;

mod api;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    stream: Option<ResponseStreamsData>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self { stream: None }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { stream } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Control Panel");

            if ui.button("GAMBA").clicked() {
                *stream = api::run().ok();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            if let Some(stream) = &self.stream {
                if let Some(ResponseStreamsData {
                    image: Some(loaded_image),
                    user_name,
                    ..
                }) = &self.stream
                {
                    ui.hyperlink(format!("https://www.twitch.tv/{}", user_name));
                    loaded_image.show(ui);
                }
                ui.heading(stream.user_name.clone());
                ui.horizontal(|ui| {
                    ui.label(stream.title.clone());
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        ui.label(RichText::new(stream.viewer_count.to_string()).color(Color32::RED));
                    });
                });
            }
        });
    }
}
