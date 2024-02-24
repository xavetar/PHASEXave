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

use super::{
    is_leap_year, sum_leap_years,
    is_overhead_year, sum_overhead_years
};

use crate::types::{
    planets::earth::calendar::{
        view::{CalendarView},
        constants::{
            days::{BASE_MONTH_SUM_DAYS, LEAP_MONTH_SUM_DAYS, OVERHEAD_MONTH_SUM_DAYS},
            year::{BASE_DAYS_YEAR, LEAP_DAYS_YEAR, OVERHEAD_YEAR}
        },
    }
};

pub fn era_days_from_date(view: CalendarView, year: u128, month: u8, day: u8) -> u128 {

    let mut days: u128 = day as u128;

    let leap_year: bool = is_leap_year(view, year);
    let leap_years: u128 = sum_leap_years(view, year);

    if !leap_year {
        days += (leap_years * LEAP_DAYS_YEAR as u128) + (((year - 1) - leap_years) * BASE_DAYS_YEAR as u128);
    } else {
        days += ((leap_years - 1) * LEAP_DAYS_YEAR as u128) + ((year - leap_years) * BASE_DAYS_YEAR as u128);
    }

    if view == CalendarView::Solar {
        let overhead_years: u128 = sum_overhead_years(view, year);

        if year % (OVERHEAD_YEAR as u128) > 0 {
            days -= overhead_years;
        } else if year % (OVERHEAD_YEAR as u128) == 0 {
            days -= overhead_years - 1;
        }
    }

    if month > 1 {
        if view == CalendarView::Solar {
            let overhead_year: bool = is_overhead_year(view, year);

            if !leap_year && !overhead_year {
                days += BASE_MONTH_SUM_DAYS[(month - 2) as usize] as u128;
            } else if leap_year && !overhead_year {
                days += LEAP_MONTH_SUM_DAYS[(month - 2) as usize] as u128;
            } else if !leap_year && overhead_year {
                days += OVERHEAD_MONTH_SUM_DAYS[(month - 2) as usize] as u128;
            } else if leap_year && overhead_year {
                days += BASE_MONTH_SUM_DAYS[(month - 2) as usize] as u128;
            } else {
                panic!("[IMPOSSIBLE]: What? (era_days_from_date -> CalendarView::Solar -> month > 1)")
            }
        } else if view == CalendarView::Julian || view == CalendarView::Gregorian {
            if !leap_year {
                days += BASE_MONTH_SUM_DAYS[(month - 2) as usize] as u128;
            } else if leap_year {
                days += LEAP_MONTH_SUM_DAYS[(month - 2) as usize] as u128;
            } else {
                panic!("[IMPOSSIBLE]: What? (era_days_from_date -> CalendarView::Julian || CalendarView::Gregorian -> month > 1)")
            }
        } else {
            panic!("[ERROR]: Unknown CalendarView (era_days_from_date)!")
        }
    }
    return days;
}