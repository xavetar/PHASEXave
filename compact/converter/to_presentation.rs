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

// This example is not for use, only as reference
// Этот пример не для использования, а для ознакомления

pub const MONTHS_IN_YEAR: u8 = 12_u8;

pub const JULIAN_BCE_DAYS_FIRST_YEAR: u8 = 2_u8;

pub const BASE_MONTH_DAYS: [u8; MONTHS_IN_YEAR as usize] = [31_u8, 28_u8, 31_u8, 30_u8, 31_u8, 30_u8, 31_u8, 31_u8, 30_u8, 31_u8, 30_u8, 31_u8];

#[derive(Default, Debug, Copy, Clone)]
pub enum CalendarView {
    #[default]
    Julian = 0,
    Gregorian = 1,
    Solar = 2
}

#[derive(Debug, Copy, Clone)]
struct Date {
    day: u8,
    month: u8,
    year: u64,
    era_days: f64,
    view: CalendarView
}

pub fn is_leap_year(view: CalendarView, year: u64) -> bool {
    match view {
        CalendarView::Julian => return year % 4_u64 == 0_u64,
        CalendarView::Gregorian => return year % 4_u64 == 0_u64 && (year % 100_u64 != 0_u64 || year % 400_u64 == 0_u64),
        CalendarView::Solar => {
            if year != 0 {
                let leap_years_fraction: f64 = ((year - 1_u64) * 24219_u64).rem_euclid(100000) as f64 / 100000.0_f64;

                if (leap_years_fraction + 0.24219_f64) as u64 > leap_years_fraction as u64 {
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

fn to_presentation(date: &mut Date, to: CalendarView) {

    let last_year: f64 = (date.year - 1_u64) as f64;

    match (date.view, to) {
        (CalendarView::Julian, CalendarView::Julian) => {
            // Equivalent to: (365.25 * last_year) - 2
            date.view = CalendarView::Julian;
            date.era_days = ((last_year * 365.0_f64) + (last_year * 25.0_f64) / 100.0_f64) - JULIAN_BCE_DAYS_FIRST_YEAR as f64;
            date.era_days += days_from(&date);
        },
        (CalendarView::Julian, CalendarView::Gregorian) => {
            // Equivalent to: 365.2425 * last_year
            // Equivalent to: ((365.25 * last_year)) - 0.0075 * last_year
            date.view = CalendarView::Gregorian;
            date.era_days = (((last_year * 365.0_f64) + (last_year * 25.0_f64) / 100.0_f64)) - ((last_year * 75.0_f64) / 10000.0_f64);
            date.era_days += days_from(&date);
        },
        (CalendarView::Julian, CalendarView::Solar) => {
            // Equivalent to: 365.24219 * last_year
            // Equivalent to: ((365.25 * last_year)) - 0.00781 * last_year
            date.view = CalendarView::Julian;
            date.era_days = (((last_year * 365.0_f64) + (last_year * 25.0_f64) / 100.0_f64)) - ((last_year * 781.0_f64) / 100000.0_f64);
            date.era_days += days_from(&date);
        },
        (CalendarView::Gregorian, CalendarView::Gregorian) => {
            // Equivalent to: 365.2425 * last_year
            date.view = CalendarView::Gregorian;
            date.era_days = (last_year * 365.0_f64) + (last_year * 2425.0_f64) / 10000.0_f64;
            date.era_days += days_from(&date);
        },
        (CalendarView::Gregorian, CalendarView::Julian) => {
            // Equivalent to: (365.25 * last_year) - 2
            // Equivalent to: ((365.2425 * last_year) + 0.0075 * last_year) - 2
            date.view = CalendarView::Julian;
            date.era_days = (((last_year * 365.0_f64) + (last_year * 2425.0_f64) / 10000.0_f64) + ((last_year * 75.0_f64) / 10000.0_f64)) - JULIAN_BCE_DAYS_FIRST_YEAR as f64;
            date.era_days += days_from(&date);
        },
        (CalendarView::Gregorian, CalendarView::Solar) => {
            // Equivalent to: 365.24219 * last_year
            // Equivalent to: (365.2425 * last_year) - 0.00031 * last_year
            date.view = CalendarView::Solar;
            date.era_days = ((last_year * 365.0_f64) + (last_year * 2425.0_f64) / 10000.0_f64) - ((last_year * 31.0_f64) / 100000.0_f64);
            date.era_days += days_from(&date);
        },
        (CalendarView::Solar, CalendarView::Solar) => {
            // Equivalent to: 365.24219 * last_year
            date.view = CalendarView::Solar;
            date.era_days = (last_year * 365.0_f64) + (last_year * 24219.0_f64) / 100000.0_f64;
            date.era_days += days_from(&date);
        },
        (CalendarView::Solar, CalendarView::Julian) => {
            // Equivalent to: 365.25 * last_year
            // Equivalent to: ((365.24219 * last_year) + 0.00781 * last_year) - 2
            date.view = CalendarView::Julian;
            date.era_days = (((last_year * 365.0_f64) + (last_year * 24219.0_f64) / 100000.0_f64) + ((last_year * 781.0_f64) / 100000.0_f64)) - JULIAN_BCE_DAYS_FIRST_YEAR as f64;
            date.era_days += days_from(&date);
        },
        (CalendarView::Solar, CalendarView::Gregorian) => {
            // Equivalent to: 365.2425 * last_year
            // Equivalent to: ((365.24219 * last_year) + 0.00031 * last_year)
            date.view = CalendarView::Gregorian;
            date.era_days = ((last_year * 365.0_f64) + (last_year * 24219.0_f64) / 100000.0_f64) + ((last_year * 31.0_f64) / 100000.0_f64);
            date.era_days += days_from(&date);
        },
        _ => panic!("[ERROR]: Unknown conversion method (to_presentation)!")
    }
}

fn check_date(date: &Date) -> bool {
    if date.month != 0 && date.month < 13_u8 {
        if date.day != 0 {
            if date.day <= BASE_MONTH_DAYS[(date.month - 1_u8) as usize] {
                return true;
            } else if date.day == 29_u8 && date.month == 2_u8 && is_leap_year(date.view, date.year) {
                return true;
            }
        }
    }
    return false;
}

fn days_from(date: &Date) -> f64 {

    if !check_date(&date) { panic!("[ERROR]: Invalid input date, or is not leap in this calendar: {:?}.", date.view); };

    let mut days: f64 = date.day as f64;

    if date.month > 1_u8 && date.month < 13_u8 {
        for m in 1_u8..date.month {
            days += BASE_MONTH_DAYS[(m - 1_u8) as usize] as f64;
        }

        // Any year, since the previous years are counted with float point (hence leap seconds/days are already included)
        if date.month > 2_u8 {
            match date.view {
                CalendarView::Julian => {
                    days += 0.25_f64;
                },
                CalendarView::Gregorian => {
                    days += 0.2425_f64;
                },
                CalendarView::Solar => {
                    days += 0.24219_f64;
                },
                _ => panic!("[ERROR]: Unknown CalendarView (days_from)!")
            }
        }
    }

    return days;
}

fn main() {
    let mut date: Date = Date {
        day: 28_u8,
        month: 2_u8,
        year: 2025_u64,
        era_days: 0.0_f64,
        view: CalendarView::Gregorian
    };

    to_presentation(&mut date, CalendarView::Solar);

    println!("Date: {:?}", date);
}