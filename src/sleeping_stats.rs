use chrono::{Datelike, Local};
use egui::widgets::Widget;
use egui::{Response, Ui};
use egui_extras::{Column, TableBuilder};

use super::dodo::Dodo;

pub struct SleepingStats<'a> {
    age: u8,
    data: &'a Vec<Dodo>,
    start_day_hour: u32,
}

impl<'a> SleepingStats<'a> {
    pub fn new(age: u8, data: &'a Vec<Dodo>) -> SleepingStats<'a> {
        SleepingStats {
            age,
            data,
            start_day_hour: 12,
        }
    }

    pub fn is_sleeping(&self, year: i32, month: u32, day: u32, hour: u32) -> bool {
        for dodo in self.data {
            if dodo.is_sleeping(year, month, day, hour) {
                return true;
            }
        }
        return false;
    }
}

impl<'a> Widget for SleepingStats<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let cur_datetime = Local::now();
        let cur_year = cur_datetime.year();
        let cur_month = cur_datetime.month();

        ui.scope(|ui| {
            let (amin, amax) = match self.age {
                0..=2 => (11,14),
                3..=5 => (10,13),
                6..=13 => (9,11),
                14..=17 => (8,10),
                18..=64 => (7,9),
                _ => (7, 8),
            };

            // ui.label("La régularité du sommeil est un prédicteur plus puissant du risque de mortalité que la durée du sommeil");
            // ui.label("Il est aussi important de maintenir des temps de sommeil similaire entre les jours que de respecter son temps de sommeil pour améliorer sa qualité de vie");
            // ui.label(format!("Par ailleurs, vous avez {} ans. Votre cycle de sommeil devrait être entre {} et {} heures par jour.", self.age, amin, amax));

            ui.label("");
            ui.label("Sleep regularity is a stronger predictor of mortality risk than sleep duration (https://doi.org/10.1093/sleep/zsad253)");
            ui.label("It is as important to maintain similar sleep times between days as it is to respect your sleep time to improve your quality of life.");
            ui.label(format!("Besides, you are {} years old. Your sleep cycle should be between {} and {} hours per day.", self.age, amin, amax));
            ui.label("");
        });

        let tb = TableBuilder::new(ui)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            // .cell_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight))
            .columns(Column::auto(), 24 + 1)
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("hour\nday");
                });
                for i in 0..24 {
                    header.col(|ui| {
                        ui.heading(format!("{:02}", (i + self.start_day_hour) % 24));
                    });
                }
            });

        tb.body(|mut body| {
            for y in 0..31 {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.add(egui::Label::new(format!("{:02}", y + 1)).wrap(false));
                    });
                    // start_day_hour to midnight
                    for h in self.start_day_hour..24 {
                        row.col(|ui| {
                            if self.is_sleeping(cur_year, cur_month, y + 1, h) {
                                ui.visuals_mut().override_text_color = Some(egui::Color32::GREEN);
                                ui.add(egui::Label::new("X").wrap(false));
                            } else {
                                // ui.visuals_mut().override_text_color = Some(egui::Color32::DARK_GRAY);
                                ui.add(egui::Label::new("").wrap(false));
                            }
                        });
                    }
                    // midnight to start next day
                    let next_day = y + 1 + 1;
                    let start_next_day = self.start_day_hour;
                    for h in 0..start_next_day {
                        row.col(|ui| {
                            if self.is_sleeping(cur_year, cur_month, next_day, h) {
                                ui.visuals_mut().override_text_color = Some(egui::Color32::GREEN);
                                ui.add(egui::Label::new("X").wrap(false));
                            } else {
                                // ui.visuals_mut().override_text_color = Some(egui::Color32::DARK_GRAY);
                                ui.add(egui::Label::new("").wrap(false));
                            }
                        });
                    }
                })
            }
        });

        ui.label("")
    }
}
