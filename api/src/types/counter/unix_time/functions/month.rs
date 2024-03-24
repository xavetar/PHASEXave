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
                functions::{is_leap_year},
                constants::{
                    months::{
                        Months, MONTHS_IN_YEAR,
                        BASE_MONTH_DAYS, LEAP_MONTH_DAYS,
                    }
                }
            }
        }
    }
};

pub fn month_from_days(view: CalendarView, year: u64, days: &mut u16) -> Months {
    let months: &[u8; MONTHS_IN_YEAR as usize];

    if !is_leap_year(view, year) {
        months = &BASE_MONTH_DAYS
    } else {
        months = &LEAP_MONTH_DAYS
    }

    let mut month: usize = 0_usize;

    loop {
        if *days > (months[month] as u16) {
            *days -= months[month] as u16;
            month += 1_usize;
        } else {
            break;
        }
    }

    return Months::from((month + 1_usize) as u8);
}