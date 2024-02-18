mod dodometterapp;
mod dodo;
mod sleeping_stats;
mod user;

use dodometterapp::DodoMeterApp;

#[cfg(target_os = "android")]
use egui_winit::winit;
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    use eframe::Renderer;

    std::env::set_var("RUST_BACKTRACE", "full");
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let options = eframe::NativeOptions {
        event_loop_builder: Some(Box::new(|builder| {
            builder.with_android_app(app);
        })),
        renderer: Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "DodoMeter",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(DodoMeterApp::new(cc))
        }),
    ).unwrap();
}
