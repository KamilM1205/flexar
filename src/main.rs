use eframe::{egui, epi};

mod about;

struct FlexApp {
    about_w: bool,
    website: Website,
    proxy_use: bool,
    proxy_url: String,
    use_custom_pas: bool,
    default_pas: String,
    pas_type: PasswordType,
    pas_len: u16,
    pas_letters: bool,
    pas_nums: bool,
    pas_file: String,
    con_photo: bool,
    reg_method: RegMethod,
    reg_num: u32,
    reg_count: u32,
}

#[derive(Debug, PartialEq)]
enum Website {
    VK,
    Instagram,
    Twitter,
    Other(String),
}

#[derive(Debug, PartialEq)]
enum RegMethod {
    Phone,
    Email,
}

#[derive(Debug, PartialEq)]
enum PasswordType {
    Generate,
    FromFile,
}

impl Default for FlexApp {
    fn default() -> Self {
        Self {
            about_w: false,
            website: Website::VK,
            proxy_use: false,
            proxy_url: String::new(),
            use_custom_pas: false,
            default_pas: String::from("Abcd5678"),
            pas_type: PasswordType::Generate,
            pas_len: 8,
            pas_letters: true,
            pas_nums: true,
            pas_file: String::from("..."),
            con_photo: true,
            reg_method: RegMethod::Phone,
            reg_num: 10,
            reg_count: 0,
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
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut font = egui::FontDefinitions::default();

        font.font_data.insert(
            "font".to_owned(),
            std::borrow::Cow::Borrowed(include_bytes!("../assets/font.ttf")),
        );
        font.fonts_for_family
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "font".to_owned());
        ctx.set_fonts(font);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Open").clicked() {};
                    if ui.button("Save").clicked() {};
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
                                .selected_text(format!("{:?}", self.website))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.website, Website::VK, "VK");
                                    ui.selectable_value(
                                        &mut self.website,
                                        Website::Instagram,
                                        "Instagram",
                                    );
                                    ui.selectable_value(
                                        &mut self.website,
                                        Website::Twitter,
                                        "Twitter",
                                    );
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Registration method: ");
                            egui::ComboBox::from_id_source("Reg sel")
                                .selected_text(format!("{:?}", self.reg_method))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.reg_method,
                                        RegMethod::Phone,
                                        "Phone",
                                    );
                                    ui.selectable_value(
                                        &mut self.reg_method,
                                        RegMethod::Email,
                                        "Email",
                                    );
                                });
                        });

                        egui::CollapsingHeader::new("Proxy")
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Use proxy: ");
                                    ui.checkbox(&mut self.proxy_use, "");
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Proxy file: ");
                                    ui.add_enabled(
                                        false,
                                        egui::TextEdit::singleline(&mut self.proxy_url),
                                    );
                                    ui.button("Select file");
                                });
                            });

                        egui::CollapsingHeader::new("Password")
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Use custom: ");
                                    ui.checkbox(&mut self.use_custom_pas, "");
                                });
                                if !self.use_custom_pas {
                                    ui.horizontal(|ui| {
                                        ui.label("Default: ");
                                        ui.add(egui::TextEdit::singleline(&mut self.default_pas));
                                    });
                                } else {
                                    ui.horizontal(|ui| {
                                        ui.label("Use: ");
                                        ui.selectable_value(
                                            &mut self.pas_type,
                                            PasswordType::Generate,
                                            "Generate",
                                        );
                                        ui.selectable_value(
                                            &mut self.pas_type,
                                            PasswordType::FromFile,
                                            "From file",
                                        );
                                        todo!();
                                    });

                                    if let PasswordType::Generate = self.pas_type {
                                        ui.horizontal(|ui| {
                                            ui.label("Password length: ");
                                            ui.add(egui::DragValue::new(&mut self.pas_len));
                                        });
                                        ui.horizontal(|ui| {
                                            ui.label("Use capital letters: ");
                                            ui.checkbox(&mut self.pas_letters, "");
                                        });
                                        ui.horizontal(|ui| {
                                            ui.label("Use numbers: ");
                                            ui.checkbox(&mut self.pas_nums, "");
                                        });
                                    } else {
                                        ui.label(format!("File path: {}", self.pas_file));
                                        ui.button("Select file");
                                    }
                                }
                            });

                        egui::CollapsingHeader::new("Account content")
                            .default_open(false)
                            .show(ui, |ui| {
                                // use photo
                                // use status
                                // subscribe
                                // posts
                                ui.horizontal(|ui| {
                                    ui.label("Use photo: ");
                                    ui.checkbox(&mut self.con_photo, "");
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Status: ");
                                    egui::ComboBox::from_id_source("accounts_status")
                                        .show_ui(ui, |ui| {});
                                });
                            });

                        ui.horizontal(|ui| {
                            ui.label("Number of accounts: ");
                            ui.add(egui::DragValue::new(&mut self.reg_num));
                        });

                        ui.label(format!("Registered: {}", self.reg_count));

                        ui.add(egui::Button::new("Start"));
                        ui.add(egui::Button::new("Stop"));
                    },
                );
            });
        });

        if self.about_w {
            about::show(ctx, &mut self.about_w);
        }
    }
}

fn main() {
    let app = FlexApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(300., 480.));
    native_options.resizable = false;
    eframe::run_native(Box::new(app), native_options);
}
