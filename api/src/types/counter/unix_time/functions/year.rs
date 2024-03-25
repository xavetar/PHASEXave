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
    planets::earth::calendar::{
        view::{CalendarView},
        constants::{
            year::{BASE_DAYS_YEAR, LEAP_DAYS_YEAR}
        },
        functions::{is_leap_year, days_from_presentation_date}
    }
};

pub fn year_from_presentation_days(view: CalendarView, presentation_days: u128) -> (u64, u16) {

    let (base_div, leap_div): (u128, u128)
        =
        (
            (presentation_days - (presentation_days % BASE_DAYS_YEAR as u128)) / BASE_DAYS_YEAR as u128,
            (presentation_days - (presentation_days % LEAP_DAYS_YEAR as u128)) / LEAP_DAYS_YEAR as u128
        );

    let (upper_limit_year, lower_limit_year): (u128, u128);

    if LEAP_DAYS_YEAR > BASE_DAYS_YEAR {
        (upper_limit_year, lower_limit_year) = (base_div, leap_div);
    } else {
        (upper_limit_year, lower_limit_year) = (leap_div, base_div);
    }

    let mut potential_year: u64;

    let (mut potential_era_days, mut fuzzy_search_year_part): (u128, u128) = (0_u128, upper_limit_year);

    loop {
        if (lower_limit_year + (fuzzy_search_year_part / 2)) > u64::MAX as u128 {
            fuzzy_search_year_part /= 2_u128;
            continue;
        }

        potential_year = (lower_limit_year + (fuzzy_search_year_part / 2)) as u64;

        if potential_year != 0_u64 {
            potential_era_days = days_from_presentation_date(view, potential_year, 0, 0);
        } else {
            potential_year = 1_u64;
            break;
        }

        if potential_era_days > presentation_days {
            fuzzy_search_year_part /= 2_u128;
        } else if potential_era_days == presentation_days {
            if potential_year > 1_u64 {
                potential_year -= 1_u64;
                potential_era_days = days_from_presentation_date(view, potential_year, 0, 0);
            }
            break;
        } else {
            break;
        }
    }

    loop {
        let delta_days: u128 = presentation_days - potential_era_days;

        if delta_days > 1_000_000_u128 {
            if LEAP_DAYS_YEAR > BASE_DAYS_YEAR {
                potential_year += (delta_days / LEAP_DAYS_YEAR as u128) as u64;
            } else {
                potential_year += (delta_days / BASE_DAYS_YEAR as u128) as u64;
            }

            potential_era_days = days_from_presentation_date(view, potential_year, 0, 0);
        } else {
            break;
        }
    }

    let mut unaccounted_days: u32 = (presentation_days - potential_era_days) as u32;

    loop {
        let leap_year: bool = is_leap_year(view, potential_year);

        if leap_year && unaccounted_days > LEAP_DAYS_YEAR as u32 {
            unaccounted_days -= LEAP_DAYS_YEAR as u32;
            potential_year += 1_u64;
        } else if !leap_year && unaccounted_days > BASE_DAYS_YEAR as u32 {
            unaccounted_days -= BASE_DAYS_YEAR as u32;
            potential_year += 1_u64;
        } else {
            break;
        }
    };

    return (potential_year, unaccounted_days as u16);
}

#[cfg(test)]
mod tests {
    use super::{
        CalendarView,
        BASE_DAYS_YEAR, LEAP_DAYS_YEAR,
        is_leap_year, days_from_presentation_date, year_from_presentation_days
    };

    #[test]
    fn fast_test_year_from_presentation_days() {
        // Проверка для небольших значений presentation_days
        assert_eq!(year_from_presentation_days(CalendarView::Gregorian, 10), (1, 10));
        assert_eq!(year_from_presentation_days(CalendarView::Gregorian, 100), (1, 100));
        assert_eq!(year_from_presentation_days(CalendarView::Gregorian, 365), (1, 365));
        //
        assert_eq!(year_from_presentation_days(CalendarView::Gregorian, 738885), (2023, 365));
        assert_eq!(year_from_presentation_days(CalendarView::Gregorian, 738886), (2024, 1));
        assert_eq!(year_from_presentation_days(CalendarView::Gregorian, 738967), (2024, 82));
        //
        // // Проверка для больших значений presentation_days
        assert_eq!(year_from_presentation_days(CalendarView::Gregorian, 10000), (28, 139));
        assert_eq!(year_from_presentation_days(CalendarView::Gregorian, 1000000), (2738, 332));
        assert_eq!(year_from_presentation_days(CalendarView::Gregorian, 1000000000), (2737908, 4));
        assert_eq!(year_from_presentation_days(CalendarView::Gregorian, 3338750015578004117), (9141187062234007, 181));
    }

    #[test]
    fn long_test_year_from_presentation_days() {
        fn old_style(view: CalendarView, presentation_days: u128) -> (u64, u16) {
            let mut year: u64 = 1_u64;
            let mut days: u128 = presentation_days;

            loop {
                let leap_year: bool = is_leap_year(view, year);

                if leap_year && days > LEAP_DAYS_YEAR as u128 {
                    days -= LEAP_DAYS_YEAR as u128;
                    year += 1_u64;
                } else if !leap_year && days > BASE_DAYS_YEAR as u128 {
                    days -= BASE_DAYS_YEAR as u128;
                    year += 1_u64;
                } else {
                    break;
                }
            };

            return (year, days as u16);
        }

        for view in [CalendarView::Solar, CalendarView::Julian, CalendarView::Gregorian] {
            let max_year_to_test: u64 = 20_000_u64;

            for test_year in 1_u64..=max_year_to_test {
                let presentation_days: u128 = days_from_presentation_date(view, test_year, 1, 1);

                assert_eq!(year_from_presentation_days(view, presentation_days), old_style(view, presentation_days));
            }
        }
    }
}