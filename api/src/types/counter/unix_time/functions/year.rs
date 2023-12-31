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
        constants::year::{BASE_DAYS_YEAR, LEAP_DAYS_YEAR},
        functions::{is_leap_year}
    }
};

const START_YEAR: u16 = 1;

pub fn year_from_days(view: CalendarView, days: u128) -> (u128, u128) {
    let mut day: u128 = days;
    let mut year: u128 = START_YEAR as u128;

    loop {
        if (day >= (BASE_DAYS_YEAR as u128)) && (day != 0) {
            if is_leap_year(view, year) {
                if day > LEAP_DAYS_YEAR as u128 {
                    day -= LEAP_DAYS_YEAR as u128;
                    year += 1;
                } else {
                    break;
                }
            } else {
                if day > BASE_DAYS_YEAR as u128 {
                    day -= BASE_DAYS_YEAR as u128;
                    year += 1;
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    };

    return (year, day);
}