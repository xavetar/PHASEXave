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

use crate::types::{
    data::date::{Date},
    planets::earth::calendar::{
        view::{CalendarView},
        constants::{
            week::{
                REPEAT_WEAK_DAY_CYCLE, Week
            },
            months::{MONTHS_IN_YEAR},
            shifts::{
                SOLAR_BASE_SAKAMOTO,
                JULIAN_BASE_SAKAMOTO,
                GREGORIAN_BASE_SAKAMOTO
            }
        },
        functions::{
            sum_leap_years
        }
    }
};

pub trait Sakamoto {
    fn week_day(&self) -> Week;
    fn from(view: CalendarView, year: u64, month: u8, day: u8) -> Week;
}

impl Sakamoto for Date {
    fn week_day(&self) -> Week {
        return <Date as Sakamoto>::from(self.view.clone(), self.year, self.month, self.day);
    }

    fn from(view: CalendarView, year: u64, month: u8, day: u8) -> Week {
        let BASE_YEAR_SHIFTS: &[u8; MONTHS_IN_YEAR as usize] = match view {
            CalendarView::Solar => &SOLAR_BASE_SAKAMOTO,
            CalendarView::Julian => &JULIAN_BASE_SAKAMOTO,
            CalendarView::Gregorian => &GREGORIAN_BASE_SAKAMOTO,
            _ => panic!("[ERROR]: Unknown CalendarView (Sakamoto).")
        };

        let mut local_year: u64 = year;

        if month < 3_u8 { local_year -= 1_u64; }

        return Week::from(((local_year + sum_leap_years(view, local_year) + BASE_YEAR_SHIFTS[(month - 1_u8) as usize] as u64 + day as u64) % REPEAT_WEAK_DAY_CYCLE as u64) as u8);
    }
}