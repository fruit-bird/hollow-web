use hollow::Prompt;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct HollowApp {
    // Example stuff:
    normal_link: String,
    conspiracy_link: String,
    output: String,
}

impl Default for HollowApp {
    fn default() -> Self {
        Self {
            normal_link: "https://en.wikipedia.org/wiki/Rumpelstiltskin".to_owned(),
            conspiracy_link: "https://en.wikipedia.org/wiki/Moon_landing_conspiracy_theories"
                .to_owned(),
            output: Default::default(),
        }
    }
}

impl HollowApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for HollowApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut font_definitions = egui::FontDefinitions::default();
            font_definitions.font_data.insert(
                "NotoSans".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/NotoSansJP-Regular.otf")),
            );
            font_definitions
                .families
                .insert(egui::FontFamily::Monospace, vec!["NotoSans".to_owned()]);
            ctx.set_fonts(font_definitions);

            ui.heading("The Corners Speak Latin");

            ui.horizontal(|ui| {
                ui.label("Normal Link:       ");
                ui.text_edit_singleline(&mut self.normal_link);
            });

            ui.horizontal(|ui| {
                ui.label("Conspiracy Link: ");
                ui.text_edit_singleline(&mut self.conspiracy_link);
            });

            let prompt = Prompt::new(&self.normal_link, &self.conspiracy_link, "ja");

            if ui.button("SeEk Truth").clicked() {
                let the_spooky = match prompt.run() {
                    Ok(entry) => entry,
                    Err(_) => "Error while converting".to_owned(),
                };
                self.output = the_spooky;
            }

            ui.horizontal(|ui| {
                ui.label("Output: ");
                ui.add(
                    egui::TextEdit::multiline(&mut self.output)
                        .desired_width(ui.available_width())
                        .desired_rows(20)
                        .font(egui::TextStyle::Monospace)
                        .frame(false),
                );
            });
        });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     // The central panel the region left after adding TopPanel's and SidePanel's
        //     ui.heading("eframe template");
        //     ui.hyperlink("https://github.com/emilk/eframe_template");
        //     ui.add(egui::github_link_file!(
        //         "https://github.com/emilk/eframe_template/blob/master/",
        //         "Source code."
        //     ));
        //     egui::warn_if_debug_build(ui);
        // });
    }
}
