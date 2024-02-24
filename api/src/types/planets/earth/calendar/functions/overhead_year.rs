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
    planets::earth::calendar::{
        view::{CalendarView},
        constants::{
            year::{OVERHEAD_YEAR}
        },
        functions::{is_leap_year},
    }
};

pub const fn is_overhead_year(view: CalendarView, year: u128) -> bool {
    match view {
        CalendarView::Julian => unimplemented!(),
        CalendarView::Gregorian => unimplemented!(),
        CalendarView::Solar => return year % (OVERHEAD_YEAR as u128) == 0,
        _ => panic!("[ERROR]: Unknown CalendarView (is_overhead_year)!")
    }
}

pub const fn is_overhead_leap_year(view: CalendarView, year: u128) -> bool {
    match view {
        CalendarView::Julian => unimplemented!(),
        CalendarView::Gregorian => unimplemented!(),
        CalendarView::Solar => return year % (OVERHEAD_YEAR as u128) == 0 && is_leap_year(view, year) == true,
        _ => panic!("[ERROR]: Unknown CalendarView (is_overhead_year)!")
    }
}

pub fn sum_overhead_years(view: CalendarView, year: u128) -> u128 {
    match view {
        CalendarView::Julian => unimplemented!(),
        CalendarView::Gregorian => unimplemented!(),
        CalendarView::Solar => ((year as f64) / 4000.0_f64 + (year as f64) / 20000.0_f64 + (year as f64) / 100000.0_f64).floor() as u128,
        _ => panic!("[ERROR]: Unknown CalendarView (sum_overhead_years)!")
    }
}

pub fn sum_overhead_leap_years(view: CalendarView, sum_overhead_years: u128) -> u128 {
    match view {
        CalendarView::Julian => unimplemented!(),
        CalendarView::Gregorian => unimplemented!(),
        CalendarView::Solar => {
            let mut count: u128 = 0;

            for overhead_year in (0..(sum_overhead_years * (OVERHEAD_YEAR as u128) + 1)).step_by(OVERHEAD_YEAR as usize) {
                if overhead_year != 0 {
                    if is_leap_year(view, overhead_year) {
                        count += 1;
                    }
                }
            }

            return count;
        },
        _ => panic!("[ERROR]: Unknown CalendarView (sum_overhead_leap_years)!")
    }
}