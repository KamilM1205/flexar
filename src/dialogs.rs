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
}

impl Default for ConfigDialog {
    fn default() -> Self {
        Self {
            files: vec![ConfigSelect::None],
            file: ConfigSelect::None,
            open: false,
        }
    }
}

impl ConfigDialog {
    pub fn show_open(&mut self, ctx: &eframe::egui::CtxRef) {
        if self.open {
            eframe::egui::Window::new("Open config file")
                .open(&mut self.open)
                .show(ctx, |ui| {
                    eframe::egui::TopBottomPanel::bottom("open_bottom").show_inside(ui, |ui| {
                        ui.with_layout(eframe::egui::Layout::centered_and_justified(eframe::egui::Direction::LeftToRight), |ui| {
                            ui.button("Open");
                        });
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
