use eframe::{egui, epi};

const APP_VER: &str = "1.0 beta";

struct FlexApp {}

impl Default for FlexApp {
    fn default() -> Self {
        Self {}
    }
}

impl epi::App for FlexApp {
    fn name(&self) -> &str {
        "Flexar"
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
        egui::TopBottomPanel::top("top").show(ctx, |ui| {});
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.label("Coded with ❤ by MUTS");
        });
    }
}

fn main() {
    let app = FlexApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(300., 480.));
    native_options.resizable = false;
    eframe::run_native(Box::new(app), native_options);
}
