use eframe::{egui, epi};

mod config;
mod dialogs;
mod plugin;

struct FlexApp {
    about_w: bool,
    conf_dialog: dialogs::ConfigDialog,
    config_file: config::Config,
    reg_count: u32,
    log: String,
}

impl Default for FlexApp {
    fn default() -> Self {
        Self {
            about_w: false,
            conf_dialog: dialogs::ConfigDialog::default(),
            config_file: config::Config::default(),
            reg_count: 0,
            log: String::from("Welcome to the FlexAR!\n"),
        }
    }
}

impl epi::App for FlexApp {
    fn name(&self) -> &str {
        "FlexAR Beta"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        self.conf_dialog.load(&mut self.log);

        plugin::get_list(&mut self.log);

        let mut font = egui::FontDefinitions::default();

        font.font_data.insert(
            "font".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
        );
        font.fonts_for_family
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "font".to_owned());
        ctx.set_fonts(font);

        let mut style = egui::Style::default();
        let mut spacing = egui::style::Spacing::default();
        spacing.item_spacing = egui::vec2(2., 5.);
        style.spacing = spacing;
        ctx.set_style(style);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        self.conf_dialog.open = true;
                    };
                    if ui.button("Save").clicked() {
                        self.conf_dialog.save = true;
                    };
                    if ui.button("Unpack plugins").clicked() {
                        let mut path = dirs::config_dir().unwrap();
                        path.push("flexar/plugins/".to_owned());
                        plugin::unpack_plugins(&path, plugin::PLUGINS.path(), &mut self.log);
                        self.log.push_str("Default plugins was unpacked.\n")
                    };
                    if ui.button("Exit").clicked() {
                        frame.quit();
                    };
                });
                if ui
                    .button("About")
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    self.about_w = true;
                }
            });
        });
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.label("Coded with ❤ by MUTS");
                },
            );
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        ui.heading("Flexar setup");
                        ui.horizontal(|ui| {
                            ui.label("Website: ");
                            egui::ComboBox::from_id_source("Website sel")
                                .selected_text(format!("{:?}", self.config_file.website))
                                .show_ui(ui, |ui| {
                                    for name in plugin::get_list(&mut self.log) {
                                        ui.selectable_value(
                                            &mut self.config_file.website,
                                            Some(name.clone()),
                                            name,
                                        );
                                    }
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Number of accounts: ");
                            ui.add(egui::DragValue::new(&mut self.config_file.reg_num));
                        });

                        ui.label(format!("Registered: {}", self.reg_count));

                        egui::ScrollArea::vertical()
                            .max_height(120.)
                            .show(ui, |ui| {
                                ui.add_enabled(false, egui::TextEdit::multiline(&mut self.log));
                            });

                        ui.add(egui::Button::new("Start"));
                        ui.add(egui::Button::new("Stop"));
                    },
                );
            });
        });

        if self.about_w {
            dialogs::about(ctx, &mut self.about_w);
        }
        let open = self.conf_dialog.show_open(ctx, &mut self.log);
        if let Some(c) = open {
            self.config_file = c;
        }
        self.conf_dialog
            .show_save(ctx, &mut self.config_file, &mut self.log);
    }
}

fn main() {
    let app = FlexApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(300., 480.));
    native_options.resizable = false;
    eframe::run_native(Box::new(app), native_options);
}
