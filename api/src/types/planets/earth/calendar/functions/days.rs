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

use super::{
    is_leap_year, sum_leap_years
};

use crate::types::{
    planets::earth::calendar::{
        view::{CalendarView},
        constants::{
            days::{BASE_MONTH_SUM_DAYS, LEAP_MONTH_SUM_DAYS},
            year::{BASE_DAYS_YEAR, LEAP_DAYS_YEAR}
        },
    }
};

pub fn days_from_presentation_date(view: CalendarView, year: u64, month: u8, day: u8) -> u128 {

    let (mut days, leap_year, leap_years): (u128, bool, u64)
    =
    (day as u128, is_leap_year(view, year), sum_leap_years(view, year));

    let (only_base_years, only_leap_years): (u64, u64);

    if !leap_year {
        (only_base_years, only_leap_years) = ((year - 1_u64) - leap_years, leap_years);
    } else {
        (only_base_years, only_leap_years) = (year - leap_years, leap_years - 1_u64);
    }

    days += only_base_years as u128 * BASE_DAYS_YEAR as u128 + only_leap_years as u128 * LEAP_DAYS_YEAR as u128;

    if month > 1_u8 {
        if !leap_year {
            days += BASE_MONTH_SUM_DAYS[(month - 2_u8) as usize] as u128;
        } else {
            days += LEAP_MONTH_SUM_DAYS[(month - 2_u8) as usize] as u128;
        }
    }

    return days;
}

#[cfg(test)]
mod tests {
    use super::{
        CalendarView,
        BASE_DAYS_YEAR, LEAP_DAYS_YEAR,
        BASE_MONTH_SUM_DAYS, LEAP_MONTH_SUM_DAYS,
        days_from_presentation_date, is_leap_year, sum_leap_years
    };

    #[test]
    fn test_days_from_presentation_date() {
        fn old_style(view: CalendarView, year: u64, month: u8, day: u8) -> u128 {
            let (mut days, leap_year, leap_years): (u128, bool, u64) = (day as u128, is_leap_year(view, year), sum_leap_years(view, year));

            if !leap_year {
                days += (leap_years as u128 * LEAP_DAYS_YEAR as u128) + (((year as u128 - 1_u128) - leap_years as u128) * BASE_DAYS_YEAR as u128);
            } else {
                days += ((leap_years as u128 - 1_u128) * LEAP_DAYS_YEAR as u128) + ((year as u128 - leap_years as u128) * BASE_DAYS_YEAR as u128);
            }

            if month > 1_u8 {
                if !leap_year {
                    days += BASE_MONTH_SUM_DAYS[(month - 2_u8) as usize] as u128;
                } else {
                    days += LEAP_MONTH_SUM_DAYS[(month - 2_u8) as usize] as u128;
                }
            }

            return days;
        }

        for view in [CalendarView::Solar, CalendarView::Julian, CalendarView::Gregorian] {
            for day in [1_u8, 11_u8, 17_u8, 23_u8, 28_u8] {
                for month in [1_u8, 3_u8, 5_u8, 8_u8, 11_u8] {
                    for year in 1..=100_000_u64 {
                        assert_eq!(days_from_presentation_date(view, year, month, day), old_style(view, year, month, day));
                    }
                }
            }
        }
    }

    #[test]
    fn test_days_from_presentation_date_euclid() {
        fn euclid_style(view: CalendarView, year: u64, month: u8, day: u8) -> u128 {
            let (mut days, leap_year, leap_years): (u128, bool, u64)
            =
            (day as u128, is_leap_year(view, year), sum_leap_years(view, year));

            let (only_base_years, only_leap_years): (u64, u64);

            if !leap_year {
                (only_base_years, only_leap_years) = ((year - 1_u64) - leap_years, leap_years);
            } else {
                (only_base_years, only_leap_years) = (year - leap_years, leap_years - 1_u64);
            }

            let (divisor_base, divisor_leap): (u64, u64)
            =
            (
                10_u64.pow(BASE_DAYS_YEAR.ilog10() + 1_u32),
                10_u64.pow(LEAP_DAYS_YEAR.ilog10() + 1_u32)
            );

            let (right_number_part_base_years, right_number_part_leap_years): (u64, u64)
            =
            (
                only_base_years % divisor_base,
                only_leap_years % divisor_leap
            );

            let (left_number_part_base_years, left_number_part_leap_years): (u64, u64)
            =
            (
                (only_base_years - right_number_part_base_years) / divisor_base,
                (only_leap_years - right_number_part_leap_years) / divisor_leap
            );

            days += (left_number_part_base_years as u128 * BASE_DAYS_YEAR as u128) * divisor_base as u128;
            days += right_number_part_base_years as u128 * BASE_DAYS_YEAR as u128;
            days += (left_number_part_leap_years as u128 * LEAP_DAYS_YEAR as u128) * divisor_leap as u128;
            days += right_number_part_leap_years as u128 * LEAP_DAYS_YEAR as u128;

            if month > 1_u8 {
                if !leap_year {
                    days += BASE_MONTH_SUM_DAYS[(month - 2_u8) as usize] as u128;
                } else {
                    days += LEAP_MONTH_SUM_DAYS[(month - 2_u8) as usize] as u128;
                }
            }

            return days;
        }

        for view in [CalendarView::Solar, CalendarView::Julian, CalendarView::Gregorian] {
            for day in [1_u8, 11_u8, 17_u8, 23_u8, 28_u8] {
                for month in [1_u8, 3_u8, 5_u8, 8_u8, 11_u8, 12_u8] {
                    for year in 1..=100_000_u64 {
                        assert_eq!(days_from_presentation_date(view, year, month, day), euclid_style(view, year, month, day));
                    }
                }
            }
        }
    }
}