use eframe::{egui, epi};

mod dialogs;

struct FlexApp {
    about_w: bool,
    conf_dialog: dialogs::ConfigDialog,
    website: Website,
    proxy_use: bool,
    proxy_files: Vec<Proxy>,
    proxy_sel: Proxy,
    use_custom_pas: bool,
    default_pas: String,
    pas_type: PasswordType,
    pas_len: u16,
    pas_letters: bool,
    pas_nums: bool,
    pas_files: Vec<PasswordFile>,
    pas_file: PasswordFile,
    acc_photo: bool,
    acc_status_files: Vec<StatusFile>,
    acc_status_file: StatusFile,
    acc_sub_files: Vec<SubscribeFile>,
    acc_sub_file: SubscribeFile,
    acc_posts_files: Vec<PostsFile>,
    acc_posts_file: PostsFile,
    reg_method: RegMethod,
    reg_num: u32,
    reg_count: u32,
    log: String,
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

#[derive(Debug, Clone, PartialEq)]
enum Proxy {
    None,
    File(String),
}

#[derive(Debug, PartialEq)]
enum PasswordType {
    Generate,
    FromFile,
}

#[derive(Debug, Clone, PartialEq)]
enum PasswordFile {
    None,
    File(String),
}

#[derive(Debug, Clone, PartialEq)]
enum StatusFile {
    None,
    File(String),
}

#[derive(Debug, Clone, PartialEq)]
enum SubscribeFile {
    None,
    File(String),
}

#[derive(Debug, Clone, PartialEq)]
enum PostsFile {
    None,
    File(String),
}

impl Default for FlexApp {
    fn default() -> Self {
        Self {
            about_w: false,
            conf_dialog: dialogs::ConfigDialog::default(),
            website: Website::VK,
            proxy_use: false,
            proxy_files: vec![Proxy::None],
            proxy_sel: Proxy::None,
            use_custom_pas: false,
            default_pas: String::from("Abcd5678"),
            pas_type: PasswordType::Generate,
            pas_len: 8,
            pas_letters: true,
            pas_nums: true,
            pas_files: vec![PasswordFile::None],
            pas_file: PasswordFile::None,
            acc_photo: false,
            acc_status_files: vec![StatusFile::None],
            acc_status_file: StatusFile::None,
            acc_sub_files: vec![SubscribeFile::None],
            acc_sub_file: SubscribeFile::None,
            acc_posts_files: vec![PostsFile::None],
            acc_posts_file: PostsFile::None,
            reg_method: RegMethod::Phone,
            reg_num: 10,
            reg_count: 0,
            log: String::from("Welcome to the FlexAR!"),
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
                    if ui.button("Open").clicked() {
                        self.conf_dialog.open = true;
                    };
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
                                if self.proxy_use {
                                    ui.horizontal(|ui| {
                                        ui.label("Proxy file: ");
                                        egui::ComboBox::from_id_source("proxy_file")
                                            .selected_text(format!("{:?}", self.proxy_sel))
                                            .show_ui(ui, |ui| {
                                                for i in 0..self.proxy_files.len() {
                                                    if let Proxy::File(f) =
                                                        self.proxy_files[i].clone()
                                                    {
                                                        ui.selectable_value(
                                                            &mut self.proxy_sel,
                                                            self.proxy_files[i].clone(),
                                                            f,
                                                        );
                                                    } else {
                                                        ui.selectable_value(
                                                            &mut self.proxy_sel,
                                                            Proxy::None,
                                                            "None",
                                                        );
                                                    }
                                                }
                                            });
                                    });
                                }
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
                                        ui.horizontal(|ui| {
                                            ui.label("File path: ");
                                            egui::ComboBox::from_id_source("pas_file")
                                                .selected_text(format!("{:?}", self.pas_file))
                                                .show_ui(ui, |ui| {
                                                    for i in 0..self.pas_files.len() {
                                                        if let PasswordFile::File(f) =
                                                            self.pas_files[i].clone()
                                                        {
                                                            ui.selectable_value(
                                                                &mut self.pas_file,
                                                                self.pas_files[i].clone(),
                                                                format!("{:?}", f),
                                                            );
                                                        } else {
                                                            ui.selectable_value(
                                                                &mut self.pas_file,
                                                                PasswordFile::None,
                                                                "None",
                                                            );
                                                        }
                                                    }
                                                });
                                        });
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
                                    ui.checkbox(&mut self.acc_photo, "");
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Status: ");
                                    let status =
                                        if let StatusFile::File(f) = self.acc_status_file.clone() {
                                            f
                                        } else {
                                            "None".to_owned()
                                        };
                                    egui::ComboBox::from_id_source("accounts_status")
                                        .selected_text(status)
                                        .show_ui(ui, |ui| {
                                            for i in 0..self.acc_status_files.len() {
                                                if let StatusFile::File(f) =
                                                    self.acc_status_files[i].clone()
                                                {
                                                    ui.selectable_value(
                                                        &mut self.acc_status_file,
                                                        self.acc_status_files[i].clone(),
                                                        f,
                                                    );
                                                } else {
                                                    ui.selectable_value(
                                                        &mut self.acc_status_file,
                                                        StatusFile::None,
                                                        "None",
                                                    );
                                                }
                                            }
                                        });
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Subscribe: ");
                                    let sub =
                                        if let SubscribeFile::File(f) = self.acc_sub_file.clone() {
                                            f
                                        } else {
                                            "None".to_owned()
                                        };
                                    egui::ComboBox::from_id_source("accounts_subscribe")
                                        .selected_text(sub)
                                        .show_ui(ui, |ui| {
                                            for i in 0..self.acc_sub_files.len() {
                                                if let SubscribeFile::File(f) =
                                                    self.acc_sub_files[i].clone()
                                                {
                                                    ui.selectable_value(
                                                        &mut self.acc_sub_file,
                                                        self.acc_sub_files[i].clone(),
                                                        f,
                                                    );
                                                } else {
                                                    ui.selectable_value(
                                                        &mut self.acc_sub_file,
                                                        SubscribeFile::None,
                                                        "None",
                                                    );
                                                }
                                            }
                                        });
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Posts: ");
                                    let posts =
                                        if let PostsFile::File(f) = self.acc_posts_file.clone() {
                                            f
                                        } else {
                                            "None".to_owned()
                                        };
                                    egui::ComboBox::from_id_source("accounts_posts")
                                        .selected_text(posts)
                                        .show_ui(ui, |ui| {
                                            for i in 0..self.acc_posts_files.len() {
                                                if let PostsFile::File(f) =
                                                    self.acc_posts_files[i].clone()
                                                {
                                                    ui.selectable_value(
                                                        &mut self.acc_posts_file,
                                                        self.acc_posts_files[i].clone(),
                                                        f,
                                                    );
                                                } else {
                                                    ui.selectable_value(
                                                        &mut self.acc_posts_file,
                                                        PostsFile::None,
                                                        "None",
                                                    );
                                                }
                                            }
                                        });
                                });
                            });

                        ui.horizontal(|ui| {
                            ui.label("Number of accounts: ");
                            ui.add(egui::DragValue::new(&mut self.reg_num));
                        });

                        ui.label(format!("Registered: {}", self.reg_count));

                        ui.add_enabled(false, egui::TextEdit::multiline(&mut self.log));

                        ui.add(egui::Button::new("Start"));
                        ui.add(egui::Button::new("Stop"));
                    },
                );
            });
        });

        if self.about_w {
            dialogs::about(ctx, &mut self.about_w);
        }
        self.conf_dialog.show_open(ctx);
    }
}

fn main() {
    let app = FlexApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(300., 480.));
    native_options.resizable = false;
    eframe::run_native(Box::new(app), native_options);
}
