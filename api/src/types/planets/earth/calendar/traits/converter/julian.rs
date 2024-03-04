/*
 * Copyright 2023 Stanislav Mikhailov (xavetar)
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
    planets::earth::calendar::{
        view::{CalendarView},
        constants::{
            days::{JULIAN_BCE_DAYS_FIRST_YEAR}
        },
        functions::{
            era_days_from_date
        }
    },
    counter::unix_time::{
        constants::{
            days::{UNIX_TIME_START_AFTER_DAY}
        },
    }
};

pub trait Julian : Converter {
    fn to_date(&mut self, tz_in_unixtime: bool);
    fn to_presentation(&mut self, tz_in_unixtime: bool);
}

impl Julian for Date {
    fn to_date(&mut self, tz_in_unixtime: bool) {
        match self.view {
            CalendarView::Julian => {
                if self.year == 1_u128 {
                    if self.month == 1_u8 {
                        if (1_u8..=JULIAN_BCE_DAYS_FIRST_YEAR).contains(&self.day) {
                            panic!("[IMPOSSIBLE]: This presentation of days is missing in CE (Current Era)! (to_date)")
                        }
                    }
                }

                self.era_days = era_days_from_date(self.view.clone(), self.year, self.month, self.day);
            },
            CalendarView::Gregorian | CalendarView::Solar => {
                self.era_days = era_days_from_date(self.view.clone(), self.year, self.month, self.day) + JULIAN_BCE_DAYS_FIRST_YEAR as u128;
            },
            _ => panic!("[ERROR]: Unknown CalendarView in Julian converter (to_date)")
        }

        self.fill_time(UNIX_TIME_START_AFTER_DAY + JULIAN_BCE_DAYS_FIRST_YEAR as u128, tz_in_unixtime);
        self.fill_date(CalendarView::Julian);

        self.era_days -= JULIAN_BCE_DAYS_FIRST_YEAR as u128;
    }

    fn to_presentation(&mut self, tz_in_unixtime: bool) {
        if self.year == 1_u128 {
            if self.month == 1_u8 {
                if (1_u8..=JULIAN_BCE_DAYS_FIRST_YEAR).contains(&self.day) {
                    panic!("[IMPOSSIBLE]: This presentation of days is missing in CE (Current Era)! (to_date)")
                }
            }
        }

        self.era_days = era_days_from_date(CalendarView::Julian, self.year, self.month, self.day);

        self.fill_time(UNIX_TIME_START_AFTER_DAY + JULIAN_BCE_DAYS_FIRST_YEAR as u128, tz_in_unixtime);
        self.fill_date(CalendarView::Julian);

        self.era_days -= JULIAN_BCE_DAYS_FIRST_YEAR as u128;
    }
}