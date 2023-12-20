use chrono::{DateTime, Datelike, Duration, Local, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dodo {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
}

impl Dodo {
    #[inline]
    pub fn is_in_good_month(&self, year: i32, month: u32) -> bool {
        self.start.year() == year && self.start.month() == month
    }

    // FIXME: last day of month (30 or 31 _must_ change month)
    pub fn is_sleeping(&self, year: i32, month: u32, day: u32, hour: u32) -> bool {
        if self.is_in_good_month(year, month) {
            let duration: Duration = self.end - self.start;
            let start_hour = self.start.hour();
            let end_hour: u32 = self.start.hour() + duration.num_hours() as u32;

            if self.start.day() == day {
                if start_hour <= hour && hour <= end_hour {
                    return true;
                }
            } else if self.end.day() == day {
                let from_midnight_to = self.end.hour();
                if hour <= from_midnight_to {
                    return true;
                }
            }
        }

        return false;
    }
}
