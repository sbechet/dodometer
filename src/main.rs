use chrono::{DateTime, Duration, Local};
use eframe::egui;
use egui::{Image, Vec2};
use serde::{Deserialize, Serialize};

use starfield::ui::StarFieldUi;

mod dodo;
use dodo::Dodo;

mod sleeping_stats;
use sleeping_stats::SleepingStats;

const SLOTHLOVE: egui::ImageSource = egui::include_image!("../images/slothlove.svg");

enum StateApp {
    Intro,
    Menu,
    Add,
    Stats,
    Config,
    About,
    Exit,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    age: u8,
    begin: Option<DateTime<Local>>,
    data: Vec<Dodo>,
}

impl Default for User {
    fn default() -> Self {
        User {
            age: 0,
            begin: None,
            data: vec![],
        }
    }
}

struct DodoMeterApp<'a> {
    state: StateApp,
    slothlove: Image<'a>,
    sfui: StarFieldUi,
    /* Intro State data */
    start_time: DateTime<Local>,
    anim_step: f32,
    /* Stats State data */
    user: User,
}

impl<'a> Default for DodoMeterApp<'a> {
    fn default() -> Self {
        Self {
            state: StateApp::Intro,
            slothlove: Image::new(SLOTHLOVE),
            sfui: StarFieldUi::default(),
            start_time: Local::now(),
            anim_step: 0.0,
            user: User::default(),
        }
    }
}

impl<'a> DodoMeterApp<'a> {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        // Pre-load for best SVG to Bitmap we want
        let _r = app
            .slothlove
            .load_for_size(&cc.egui_ctx, Vec2::new(520.0, 444.0));

        // load user data
        if cc.storage.is_some() {
            let data = cc.storage.unwrap().get_string("dodometer");
            app.user = match data {
                Some(d) => match serde_json::from_str(&d) {
                    Ok(json) => match serde_json::from_value(json) {
                        Ok(au) => au,
                        _ => User::default(),
                    },
                    _ => User::default(),
                },
                _ => User::default(),
            };
        }
        // println!("load: {:?}", app.user);
        cc.egui_ctx
            .send_viewport_cmd(egui::ViewportCommand::Title("D0D0 M3T3R".to_string()));
        app
    }
}

impl<'a> eframe::App for DodoMeterApp<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.user.age == 0 {
            self.state = StateApp::Config;
        }
        match self.state {
            StateApp::Intro => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.ctx().request_repaint();
                    ui.vertical_centered(|ui| {
                        let step = 0.8 / (2.0 * 60.0);
                        if self.anim_step <= 0.8 {
                            self.anim_step += step;
                        }
                        ui.add_sized(ui.available_size() * self.anim_step, self.slothlove.clone());
                    });
                    self.sfui.background_rect(ui);
                });
                let w8time = Duration::seconds(2);
                if self.start_time + w8time < Local::now() {
                    self.state = StateApp::Menu;
                }
            }
            StateApp::Menu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.ctx().request_repaint();

                    ui.horizontal(|ui| {
                        ui.add_sized(
                            Vec2::splat(ui.available_size().min_elem()),
                            self.slothlove.clone(),
                        );
                        ui.heading("Dodometer");
                    });
                    ui.separator();

                    /****/
                    let (sleeping, add_text) = match self.user.begin {
                        Some(d) => {
                            let duration = Local::now() - d;
                            if duration.num_seconds() >= 2 {
                                ui.label(format!(
                                    "Sleeping since {} seconds",
                                    duration.num_seconds()
                                ));
                            }
                            (true, "Stop sleeping")
                        }
                        None => (false, "Start Sleeping"),
                    };
                    if ui.button(add_text).clicked() {
                        self.state = StateApp::Add;
                    }
                    if sleeping {
                        return;
                    }
                    /****/
                    if ui.button("Stats").clicked() {
                        self.state = StateApp::Stats;
                    }
                    if ui.button("Config").clicked() {
                        self.state = StateApp::Config;
                    }
                    if ui.button("About").clicked() {
                        self.state = StateApp::About;
                    }
                    if ui.button("Exit").clicked() {
                        self.state = StateApp::Exit;
                    }

                    self.sfui.background_rect(ui);
                });
            }
            StateApp::Add => {
                // Not a GUI state
                match self.user.begin {
                    Some(d) => {
                        let dodo = Dodo {
                            start: d,
                            end: Local::now(),
                        };
                        let duration = dodo.end - dodo.start;
                        if Duration::minutes(1) <= duration && duration < Duration::hours(11) {
                            // Add only beetween [1 minute;11 hours[
                            self.user.data.push(dodo);
                        }
                        self.user.begin = None;
                    }
                    None => {
                        self.user.begin = Some(Local::now());
                    }
                }
                self.state = StateApp::Menu;
            }
            StateApp::Stats => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.ctx().request_repaint();
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            Vec2::splat(ui.available_size().min_elem()),
                            self.slothlove.clone(),
                        );
                        ui.heading("Stats");
                    });
                    ui.separator();
                    if ui.button("Back").clicked() {
                        self.state = StateApp::Menu;
                        println!("{:?}", self.user);
                    }
                    let stats = SleepingStats::new(self.user.age, &self.user.data);
                    ui.add(stats);
                    self.sfui.background_rect(ui);
                });
            }
            StateApp::Config => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.ctx().request_repaint();
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            Vec2::splat(ui.available_size().min_elem()),
                            self.slothlove.clone(),
                        );
                        ui.heading("Configuration");
                    });
                    ui.separator();
                    if ui.button("Back").clicked() {
                        self.state = StateApp::Menu;
                    }
                    ui.label("How old are you?");
                    ui.add(egui::Slider::new(&mut self.user.age, 0..=99));
                    ui.separator();
                    ui.label("Starfield:");
                    ui.add(
                        egui::Slider::new(&mut self.sfui.sf.rotation_frame.x, -0.001..=0.001)
                            .text("X"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.sfui.sf.rotation_frame.y, -0.001..=0.001)
                            .text("Y"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.sfui.sf.rotation_frame.z, -0.001..=0.001)
                            .text("Z"),
                    );

                    self.sfui.background_rect(ui);
                });
            }
            StateApp::About => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.ctx().request_repaint();
                    if ui.button("Back").clicked() {
                        self.state = StateApp::Menu;
                    }
                    ui.centered_and_justified(|ui| {
                        ui.label("On an original idea from one of my daughters, F. previously on a paper.");
                    });
                    self.sfui.background_rect(ui);
                });
            }
            StateApp::Exit => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // save user data
        let data = serde_json::to_string_pretty(&self.user).unwrap();
        storage.set_string("dodometer", data);
        storage.flush();
        // println!("save: {:?}", self.user);
    }
}

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
