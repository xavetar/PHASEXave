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
    planets::{
        earth::{
            calendar::{
                view::{CalendarView},
                constants::{
                    year::{
                        SOLAR_YEAR_LEAP_LENGTH_INT
                    }
                },
            }
        }
    }
};

pub const fn is_leap_year(view: CalendarView, year: u64) -> bool {
    match view {
        CalendarView::Julian => return year % 4_u64 == 0_u64,
        CalendarView::Gregorian => return year % 4_u64 == 0_u64 && (year % 100_u64 != 0_u64 || year % 400_u64 == 0_u64),
        // Equivalent to (Overflow):
        //      let leap_years_fraction: f64 = ((year - 1_u64) * SOLAR_YEAR_LEAP_LENGTH_INT).rem_euclid(100000_u64) as f64 / 100000.0_f64
        // Equivalent to (truncating division, but float point):
        //      let leap_years_fraction: f64 = ((((year - 1_u64) % 100000_u64) * SOLAR_YEAR_LEAP_LENGTH_INT) % 100000_u64) as f64 / 100000.0_f64;
        CalendarView::Solar => {
            if year != 0 {
                if sum_leap_years(view, year) > sum_leap_years(view, year - 1) {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false
            }
        },
        _ => panic!("[ERROR]: Unknown CalendarView (is_leap_year)!")
    }
}

pub const fn sum_leap_years(view: CalendarView, year: u64) -> u64 {
    match view {
        CalendarView::Julian => return year / 4_u64,
        CalendarView::Gregorian => return year / 4_u64 - year / 100_u64 + year / 400_u64,
        // Equivalent to (overflow): (year * SOLAR_YEAR_LEAP_LENGTH_INT) / 100000_u64
        // Equivalent to (truncating division):
        //      let leap_years: u64 = year.div_euclid(100000_u64) * SOLAR_YEAR_LEAP_LENGTH_INT;
        //      let indivisible_years: u64 = ((year % 100000_u64) * SOLAR_YEAR_LEAP_LENGTH_INT) / 100000_u64
        //      return leap_years + indivisible;
        // Equivalent to (truncating division):
        //      return (year.div_euclid(100000_u64) * SOLAR_YEAR_LEAP_LENGTH_INT) + (((year % 100000_u64) * SOLAR_YEAR_LEAP_LENGTH_INT) / 100000_u64)
        CalendarView::Solar => return (((year - (year % 100000_u64)) / 100000_u64) * SOLAR_YEAR_LEAP_LENGTH_INT) + (((year % 100000_u64) * SOLAR_YEAR_LEAP_LENGTH_INT) / 100000_u64),
        _ => panic!("[ERROR]: Unknown CalendarView (sum_leap_years)!")
    }
}