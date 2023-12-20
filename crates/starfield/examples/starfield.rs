use eframe::egui;
use starfield::ui::StarFieldUi;

struct MyApp {
    sfui: StarFieldUi,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            sfui: StarFieldUi::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.ctx().request_repaint();
            ui.heading("StarField Demo");
            ui.add(egui::Slider::new(&mut self.sfui.sf.rotation_frame.x, -0.001..=0.001).text("X"));
            ui.add(egui::Slider::new(&mut self.sfui.sf.rotation_frame.y, -0.001..=0.001).text("Y"));
            ui.add(egui::Slider::new(&mut self.sfui.sf.rotation_frame.z, -0.001..=0.001).text("Z"));
            self.sfui.background(ui);
        });
    }
}


fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions::default();
    let myapp = MyApp::default();

    eframe::run_native("Starfield Demo", options, Box::new(|_cc| Box::new(myapp)))?;

    Ok(())
}
