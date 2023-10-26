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

use crate::types::{
    data::date::{Date},
    planets::earth::calendar::{
        view::{CalendarView},
        constants::{
            week::{Week, REPEAT_WEAK_DAY_CYCLE},
            months::{MONTHS_IN_YEAR},
            shifts::{
                JULIAN_BASE_XAVETAR, JULIAN_LEAP_XAVETAR,
                GREGORIAN_BASE_XAVETAR, GREGORIAN_LEAP_XAVETAR
            }
        },
        functions::{
            is_leap_year_gregorian, sum_leap_years_gregorian,
            is_leap_year_julian, sum_leap_years_julian
        }
    }
};

pub trait Xavetar {
    fn week_day(&self) -> Week;
    fn from(view: CalendarView, year: u128, month: u8, day: u8) -> Week;
}

impl Xavetar for Date {
    fn week_day(&self) -> Week {
        return <Date as Xavetar>::from(self.view.clone(), self.year, self.month, self.day);
    }

    fn from(view: CalendarView, year: u128, month: u8, day: u8) -> Week {
        let ((is_leap_year, sum_leap_years), (BASE_YEAR_SHIFTS, LEAP_YEAR_SHIFTS)): ((fn(u128) -> bool, fn(u128) -> u128), (&[u8; MONTHS_IN_YEAR as usize], &[u8; MONTHS_IN_YEAR as usize])) = match view {
            CalendarView::Gregorian => ((is_leap_year_gregorian, sum_leap_years_gregorian), (&GREGORIAN_BASE_XAVETAR, &GREGORIAN_LEAP_XAVETAR)),
            CalendarView::Julian => ((is_leap_year_julian, sum_leap_years_julian), (&JULIAN_BASE_XAVETAR, &JULIAN_LEAP_XAVETAR)),
            _ => panic!("[ERROR]: Unknown CalendarView (Xavetar).")
        };

        let last_year: u128 = year - 1;

        if !is_leap_year(year) {
            return Week::from(((last_year + sum_leap_years(last_year) + BASE_YEAR_SHIFTS[(month - 1) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u128) as u8);
        } else {
            return Week::from(((last_year + sum_leap_years(last_year) + LEAP_YEAR_SHIFTS[(month - 1) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u128) as u8);
        }
    }
}