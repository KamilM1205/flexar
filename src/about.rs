pub fn show(ctx: &eframe::egui::CtxRef, open: &mut bool) {
    eframe::egui::Window::new("About").open(open).show(ctx, |ui| {
        ui.label("Test");
    });
}
