pub use chrono;
pub use chrono::{Datelike, NaiveDate, Weekday};

pub mod wd;

pub fn middle_day(year: i32) -> Option<Weekday> {
    // Determine if the year is a leap year
    let is_leap_year = NaiveDate::from_ymd_opt(year, 2, 29).is_some();

    // Calculate total days in the year
    let total_days = if is_leap_year { 366 } else { 365 };

    // Only years with odd number of days have a middle day
    if total_days % 2 == 0 {
        return None;
    }

    // Calculate the middle day (integer division)
    let middle_day_number = (total_days / 2) + 1;

    // Create a date for the middle day and get its weekday
    if let Some(date) = NaiveDate::from_yo_opt(year, middle_day_number as u32) {
        Some(date.weekday())
    } else {
        None
    }
}