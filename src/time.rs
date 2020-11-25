use crate::prelude::*;
#[derive(Debug, PartialEq)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

#[derive(Debug, PartialEq)]
pub struct Zone {
    pub sign: bool,
    pub hour_offset: u8,
    pub minute_offset: u8,
}

#[derive(Debug, PartialEq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(Debug, PartialEq)]
pub struct TimeWithZone {
    pub time: Time,
    pub zone: Zone,
}

#[derive(Debug, PartialEq)]
pub struct Date {
    pub day: u8,
    pub month: Month,
    pub year: usize,
}

#[derive(Debug, PartialEq)]
pub struct DateTime {
    pub day_name: Option<Day>,
    pub date: Date,
    pub time: TimeWithZone,
}

impl DateTime {
    fn is_leap_year(year: u16) -> bool {
        // 1.能被4整除,但不能被100整除 2能被400整除
        if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
            return true;
        }
        false
    }

    fn days_of_the_month(year: u16, month: u8) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if DateTime::is_leap_year(year) {
                    return 29;
                }
                28
            }
            _ => 0,
        }
    }

    pub fn get_timestamp(&self) -> Result<u32, Error> {
        if self.date.year < 1970 {
            return Err(Error::Known("year less than 1970"));
        }
        let second = self.time.time.second as u32;
        let minute = self.time.time.minute as u32;
        let hour = self.time.time.hour as u32;
        let day = self.date.day as u32;
        let year: u32 = self.date.year as u32;

        let seconds_this_month = second + 60 * minute + 60 * 60 * hour + 60 * 60 * 24 * (day - 1);
        let mut seconds_past_years: u32 = 0;
        for i in 1970..year {
            if DateTime::is_leap_year(i as u16) {
                seconds_past_years += 366 * 24 * 60 * 60;
            } else {
                seconds_past_years += 365 * 24 * 60 * 60;
            }
        }

        let mut seconds_past_months = 0;
        for i in 1..self.date.month as u8 {
            let days_num = DateTime::days_of_the_month(year as u16, i) as u32;
            seconds_past_months += days_num * 24 * 60 * 60;
        }

        let timestamp = seconds_past_years + seconds_past_months + seconds_this_month;
        // Ok(timestamp)

        let minute_offset = self.time.zone.minute_offset as u32;
        let hour_offset = self.time.zone.hour_offset as u32;
        let offset = (hour_offset * 60 + minute_offset) * 60;
        match self.time.zone.sign {
            true => Ok(timestamp - offset),
            false => Ok(timestamp + offset),
        }
    }
}
