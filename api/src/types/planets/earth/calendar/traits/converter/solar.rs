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
            days::{ALIGN_JULIAN}
        },
        functions::{
            days_from_date, era_days_from_date
        }
    },
    counter::unix_time::{
        constants::{
            days::{UNIX_TIME_START_AFTER_DAY}
        },
    }
};

pub trait Solar : Converter {
    fn to_date(&mut self, tz_in_unixtime: bool);
    fn to_presentation(&mut self, tz_in_unixtime: bool);
}

impl Solar for Date {
    fn to_date(&mut self, tz_in_unixtime: bool) {
        self.era_days = era_days_from_date(self.view.clone(), self.year, self.month, self.day);

        match self.view {
            CalendarView::Julian => {
                if self.era_days >= ALIGN_JULIAN as u128 {
                    self.era_days -= ALIGN_JULIAN as u128;
                } else {
                    panic!("[IMPOSSIBLE]: This days is missing in Solar Calendar! (to_date)")
                }
            },
            CalendarView::Gregorian => (),
            CalendarView::Solar => (),
            _ => panic!("[ERROR]: Unknown CalendarView in Solar converter (to_date)")
        }

        self.fill_time(UNIX_TIME_START_AFTER_DAY, tz_in_unixtime);
        self.fill_date(CalendarView::Solar);
    }

    fn to_presentation(&mut self, tz_in_unixtime: bool) {
        let last_year: u128 = if self.year > 0 { self.year - 1 } else { 0 };
        
        match self.view {
            CalendarView::Julian => {
                // Конвертируем из Юлианского в Солнечный (в Юлианском больше високосных дней, но представление меньше)
                // Удаляем из Юлианских дней эры, отсутствующие в Солнечном календаре дни Юлианского календаря
                self.era_days = 365_u128 * last_year;
                self.era_days += ((last_year * 25_u128) as f64 / 100.0_f64 - ((last_year * 781_u128) as f64 / 100000.0_f64)) as u128;
                self.era_days += days_from_date(CalendarView::Solar, self.year, self.month, self.day);
            },
            CalendarView::Gregorian => {
                // Конвертируем из Григорианского в Солнечный (в Григорианском больше високосных дней, но представление меньше)
                // Удаляем из Григорианских дней эры, отсутствующие в Солнечном календаре дни Григорианского календаря
                self.era_days = 365_u128 * last_year;
                self.era_days += ((last_year * 2425_u128) as f64 / 10000.0_f64 - ((last_year * 31_u128) as f64 / 100000.0_f64)) as u128;
                self.era_days += days_from_date(CalendarView::Solar, self.year, self.month, self.day);
            },
            CalendarView::Solar => {
                self.era_days = era_days_from_date(self.view.clone(), self.year, self.month, self.day);
            },
            _ => panic!("[ERROR]: Unknown CalendarView in Solar converter (to_presentation)")
        }

        self.fill_time(UNIX_TIME_START_AFTER_DAY, tz_in_unixtime);
        self.fill_date(CalendarView::Solar);
    }
}