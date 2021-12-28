pub fn about(ctx: &eframe::egui::CtxRef, open: &mut bool) {
    eframe::egui::Window::new("About")
        .open(open)
        .show(ctx, |ui| {
            ui.label("Test");
        });
}

#[derive(Debug, Clone, PartialEq)]
enum ConfigSelect {
    None,
    File(String),
}

pub struct ConfigDialog {
    files: Vec<ConfigSelect>,
    file: ConfigSelect,
    pub open: bool,
    close_open: bool,
    pub save: bool,
    close_save: bool,
    save_name: String,
}

impl Default for ConfigDialog {
    fn default() -> Self {
        Self {
            files: vec![ConfigSelect::None],
            file: ConfigSelect::None,
            open: false,
            close_open: false,
            save: false,
            close_save: false,
            save_name: String::new(),
        }
    }
}

impl ConfigDialog {
    pub fn show_save(&mut self, ctx: &eframe::egui::CtxRef, log: &mut String) {
        if self.close_save {
            self.close_save = false;
            self.save = false;
        }
        if self.save {
            eframe::egui::Window::new("Save config file")
                .open(&mut self.save)
                .show(ctx, |ui| {
                    eframe::egui::TopBottomPanel::bottom("save_bottom").show_inside(ui, |ui| {
                        ui.with_layout(
                            eframe::egui::Layout::centered_and_justified(
                                eframe::egui::Direction::LeftToRight,
                            ),
                            |ui| {
                                if ui.button("Save").clicked() && self.save_name != "" {
                                    self.close_save = true;
                                    log.push_str(&format!(
                                        "Config file {} was saved.\n",
                                        self.save_name
                                    ));
                                }
                            },
                        );
                    });
                    eframe::egui::TopBottomPanel::top("save_top").show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Save name: ");
                            ui.add(eframe::egui::TextEdit::singleline(&mut self.save_name));
                        });
                    });
                });
        }
    }
    pub fn show_open(&mut self, ctx: &eframe::egui::CtxRef, log: &mut String) {
        if self.close_open {
            self.close_open = false;
            self.open = false;
        }
        if self.open {
            eframe::egui::Window::new("Open config file")
                .open(&mut self.open)
                .show(ctx, |ui| {
                    eframe::egui::TopBottomPanel::bottom("open_bottom").show_inside(ui, |ui| {
                        ui.with_layout(
                            eframe::egui::Layout::centered_and_justified(
                                eframe::egui::Direction::LeftToRight,
                            ),
                            |ui| {
                                if ui.button("Open").clicked() {
                                    if let ConfigSelect::File(_) = self.file {
                                        self.close_open = true;
                                        log.push_str(&format!(
                                            "Config file {} was loaded.\n",
                                            self.save_name
                                        ));
                                    }
                                }
                            },
                        );
                    });
                    eframe::egui::TopBottomPanel::top("open_top").show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Select file: ");
                            eframe::egui::ComboBox::from_id_source("select config")
                                .selected_text(format!("{:?}", self.file))
                                .show_ui(ui, |ui| {
                                    for i in 0..self.files.len() {
                                        let fname = if let ConfigSelect::File(fname) =
                                            self.files[i].clone()
                                        {
                                            fname
                                        } else {
                                            "None".to_owned()
                                        };
                                        if let ConfigSelect::File(f) = self.files[i].clone() {
                                            ui.selectable_value(
                                                &mut self.file,
                                                self.files[i].clone(),
                                                fname,
                                            );
                                        } else {
                                            ui.selectable_value(
                                                &mut self.file,
                                                ConfigSelect::None,
                                                "None",
                                            );
                                        }
                                    }
                                });
                        });
                    });
                });
        }
    }
}
