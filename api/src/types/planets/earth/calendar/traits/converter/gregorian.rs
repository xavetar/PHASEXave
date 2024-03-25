/*
 * Copyright 2024 Stanislav Mikhailov (xavetar)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

use super::{Converter};

use crate::types::{
    data::{
        date::{Date},
    },
    planets::{
        earth::{
            calendar::{
                view::{CalendarView},
                constants::{
                    days::{JULIAN_BCE_DAYS_FIRST_YEAR}
                },
                functions::{
                    days_from_presentation_date
                },
            }
        }
    },
    counter::{
        unix_time::{
            constants::{
                days::{UNIX_TIME_START_AFTER_DAY}
            },
        }
    }
};

pub trait Gregorian : Converter {
    fn to_date(&mut self, timezone_in_unix_time: bool);
    fn to_presentation(&mut self, timezone_in_unix_time: bool);
}

impl Gregorian for Date {
    fn to_date(&mut self, timezone_in_unix_time: bool) {
        self.era_days = days_from_presentation_date(self.view.clone(), self.year, self.month, self.day);

        match self.view {
            CalendarView::Julian => {
                if self.era_days > JULIAN_BCE_DAYS_FIRST_YEAR {
                    self.era_days -= JULIAN_BCE_DAYS_FIRST_YEAR;
                } else {
                    panic!("[IMPOSSIBLE]: This days is missing in CE (Current era) of Gregorian Calendar! (to_date)")
                }
            },
            CalendarView::Gregorian => (),
            CalendarView::Solar => (),
            _ => panic!("[ERROR]: Unknown CalendarView in Gregorian converter (to_date)")
        }

        self.fill_time(UNIX_TIME_START_AFTER_DAY, timezone_in_unix_time);
        self.fill_date(CalendarView::Gregorian);
    }

    fn to_presentation(&mut self, timezone_in_unix_time: bool) {
        self.era_days = days_from_presentation_date(CalendarView::Gregorian, self.year, self.month, self.day);

        self.fill_time(UNIX_TIME_START_AFTER_DAY, timezone_in_unix_time);
        self.fill_date(CalendarView::Gregorian);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CalendarView, Date,
        JULIAN_BCE_DAYS_FIRST_YEAR
    };

    use crate::{
        Months,
        Solar, Julian, Gregorian,
        functions::{
            is_leap_year
        }
    };

    #[test]
    fn test_between_presentation_conversion() {
        let mut date: Date = Date::default();

        let max_year_to_test: u64 = 10_000_u64;

        for year in 1_u64..=max_year_to_test {
            for month in [
                Months::January, Months::February, Months::March, Months::April, Months::May, Months::June,
                Months::July, Months::August, Months::September, Months::October, Months::November, Months::December
            ] {
                for day in 1_u8..=month.days(is_leap_year(CalendarView::Gregorian, year)) {

                    (date.day, date.month, date.year, date.view) = (day, month.index(), year, CalendarView::Gregorian);
                    <Date as Gregorian>::to_presentation(&mut date, false);
                    assert_eq!((date.day, date.month, date.year, date.view), (day, month.index(), year, CalendarView::Gregorian));

                    let unix_time: u128 = date.unix_time;

                    if day != 29 && month.index() != Months::February.index() {
                        <Date as Solar>::to_presentation(&mut date, true);
                        assert_eq!((date.day, date.month, date.year, date.view), (day, month.index(), year, CalendarView::Solar));

                        <Date as Gregorian>::to_presentation(&mut date, true);
                        assert_eq!((date.day, date.month, date.year, date.unix_time, date.view), (day, month.index(), year, unix_time, CalendarView::Gregorian));

                        if !(1_u128..=JULIAN_BCE_DAYS_FIRST_YEAR).contains(&(day as u128)) || month.index() != Months::January.index() || year != 1 {
                            <Date as Julian>::to_presentation(&mut date, true);
                            assert_eq!((date.day, date.month, date.year, date.view), (day, month.index(), year, CalendarView::Julian));

                            <Date as Gregorian>::to_presentation(&mut date, true);
                            assert_eq!((date.day, date.month, date.year, date.unix_time, date.view), (day, month.index(), year, unix_time, CalendarView::Gregorian));
                        }
                    }
                }
            }
        }
    }
}