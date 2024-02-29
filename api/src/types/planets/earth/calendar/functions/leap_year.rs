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
            year::{
                SOLAR_YEAR_LEAP_LENGTH_INT,
                SOLAR_YEAR_LEAP_LENGTH_F64
            }
        }
    }
};

pub fn is_leap_year(view: CalendarView, year: u128) -> bool {
    match view {
        CalendarView::Julian => return year % 4_u128 == 0_u128,
        CalendarView::Gregorian => return year % 4_u128 == 0_u128 && (year % 100_u128 != 0_u128 || year % 400_u128 == 0_u128),
        CalendarView::Solar => {
            if year != 0 {
                let leap_years: f64 = ((year - 1_u128) * SOLAR_YEAR_LEAP_LENGTH_INT) as f64 / 100000.0_f64;

                if (leap_years + SOLAR_YEAR_LEAP_LENGTH_F64) as u128 > (leap_years) as u128 {
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

pub fn sum_leap_years(view: CalendarView, year: u128) -> u128 {
    match view {
        CalendarView::Julian => return year / 4_u128,
        CalendarView::Gregorian => return year / 4_u128 - year / 100_u128 + year / 400_u128,
        CalendarView::Solar => return (year * SOLAR_YEAR_LEAP_LENGTH_INT) / 100000_u128,
        _ => panic!("[ERROR]: Unknown CalendarView (sum_leap_years)!")
    }
}

pub const fn excess_leap_years(year: u128) -> u128 {
    return year / 100_u128 - year / 400_u128;
}