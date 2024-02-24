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
                GREGORIAN_BASE_XAVETAR, GREGORIAN_LEAP_XAVETAR,
                SOLAR_BASE_XAVETAR, SOLAR_LEAP_XAVETAR, SOLAR_OVERHEAD_XAVETAR
            }
        },
        functions::{
            is_leap_year, sum_leap_years, is_overhead_year, sum_overhead_years
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
        if view == CalendarView::Solar {
            let (BASE_YEAR_SHIFTS, LEAP_YEAR_SHIFTS, OVERHEAD_YEAR_SHIFTS): (&[u8; MONTHS_IN_YEAR as usize], &[u8; MONTHS_IN_YEAR as usize], &[u8; MONTHS_IN_YEAR as usize]) = match view {
                CalendarView::Solar => (&SOLAR_BASE_XAVETAR, &SOLAR_LEAP_XAVETAR, &SOLAR_OVERHEAD_XAVETAR),
                _ => panic!("[ERROR]: Unknown CalendarView for Solar calendar (Xavetar).")
            };

            let last_year: u128 = year - 1;

            let (leap_year, overhead_year): (bool, bool) = (is_leap_year(view, year), is_overhead_year(view, year));

            if !leap_year && !overhead_year {
                return Week::from(((last_year + sum_leap_years(view, last_year) - sum_overhead_years(view, last_year) + BASE_YEAR_SHIFTS[(month - 1) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u128) as u8);
            } else if leap_year && !overhead_year {
                return Week::from(((last_year + sum_leap_years(view, last_year) - sum_overhead_years(view, last_year) + LEAP_YEAR_SHIFTS[(month - 1) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u128) as u8);
            } else if !leap_year && overhead_year {
                return Week::from(((last_year + sum_leap_years(view, last_year) - sum_overhead_years(view, last_year) + OVERHEAD_YEAR_SHIFTS[(month - 1) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u128) as u8);
            } else if leap_year && overhead_year {
                return Week::from(((last_year + sum_leap_years(view, last_year) - sum_overhead_years(view, last_year) + BASE_YEAR_SHIFTS[(month - 1) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u128) as u8);
            } else {
                panic!("[IMPOSSIBLE]: What? Unknown error (Xavetar).")
            }
        } else if view == CalendarView::Gregorian || view == CalendarView::Julian {
            let (BASE_YEAR_SHIFTS, LEAP_YEAR_SHIFTS): (&[u8; MONTHS_IN_YEAR as usize], &[u8; MONTHS_IN_YEAR as usize]) = match view {
                CalendarView::Gregorian => (&GREGORIAN_BASE_XAVETAR, &GREGORIAN_LEAP_XAVETAR),
                CalendarView::Julian => (&JULIAN_BASE_XAVETAR, &JULIAN_LEAP_XAVETAR),
                _ => panic!("[ERROR]: Unknown CalendarView for Julian or Gregorian calendar (Xavetar).")
            };

            let last_year: u128 = year - 1;

            if !is_leap_year(view, year) {
                return Week::from(((last_year + sum_leap_years(view, last_year) + BASE_YEAR_SHIFTS[(month - 1) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u128) as u8);
            } else {
                return Week::from(((last_year + sum_leap_years(view, last_year) + LEAP_YEAR_SHIFTS[(month - 1) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u128) as u8);
            }
        } else {
            panic!("[ERROR]: Unknown CalendarView (Xavetar).")
        }
    }
}