mod dodometterapp;
mod dodo;
mod sleeping_stats;
mod user;

use dodometterapp::DodoMeterApp;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "DodoMeter",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(DodoMeterApp::new(cc))
        }),
    )?;

    Ok(())
}
