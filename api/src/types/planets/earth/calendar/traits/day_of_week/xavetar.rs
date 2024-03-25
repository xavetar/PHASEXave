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
    data::{
        date::{Date}
    },
    planets::{
        earth::{
            calendar::{
                view::{CalendarView},
                constants::{
                    week::{
                        Week,
                        REPEAT_WEAK_DAY_CYCLE
                    },
                    months::{MONTHS_IN_YEAR},
                    shifts::{
                        JULIAN_BASE_XAVETAR, JULIAN_LEAP_XAVETAR,
                        SOLAR_BASE_XAVETAR, SOLAR_LEAP_XAVETAR,
                        GREGORIAN_BASE_XAVETAR, GREGORIAN_LEAP_XAVETAR,
                    },
                },
                functions::{
                    is_leap_year, sum_leap_years,
                },
            }
        }
    }
};

pub trait Xavetar {
    fn week_day(&self) -> Week;
    fn from(view: CalendarView, year: u64, month: u8, day: u8) -> Week;
}

impl Xavetar for Date {
    fn week_day(&self) -> Week {
        return <Date as Xavetar>::from(self.view.clone(), self.year, self.month, self.day);
    }

    fn from(view: CalendarView, year: u64, month: u8, day: u8) -> Week {
        let (BASE_YEAR_SHIFTS, LEAP_YEAR_SHIFTS): (&[u8; MONTHS_IN_YEAR as usize], &[u8; MONTHS_IN_YEAR as usize]) = match view {
            CalendarView::Solar => (&SOLAR_BASE_XAVETAR, &SOLAR_LEAP_XAVETAR),
            CalendarView::Julian => (&JULIAN_BASE_XAVETAR, &JULIAN_LEAP_XAVETAR),
            CalendarView::Gregorian => (&GREGORIAN_BASE_XAVETAR, &GREGORIAN_LEAP_XAVETAR),
            _ => panic!("[ERROR]: Unknown CalendarView for Julian or Gregorian calendar (Xavetar).")
        };

        let last_year: u64 = year - 1_u64;

        return if !is_leap_year(view, year) {
            Week::from(((last_year as u128 + sum_leap_years(view, last_year) as u128 + BASE_YEAR_SHIFTS[(month - 1_u8) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u128) as u8)
        } else {
            Week::from(((last_year as u128 + sum_leap_years(view, last_year) as u128 + LEAP_YEAR_SHIFTS[(month - 1_u8) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u128) as u8)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CalendarView, Week, Xavetar
    };

    use crate::types::{
        data::{
            date::{Date},
        },
        counter::{
            unix_time::{
                functions::{
                    year_from_presentation_days, month_from_days,
                }
            }
        },
        planets::{
            earth::{
                calendar::{
                    constants::{
                        months::{Months::December},
                        week::{
                            SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_SOLAR,
                            SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_JULIAN,
                            SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_GREGORIAN,
                        },
                    },
                    functions::{
                        is_leap_year,
                        days_from_presentation_date
                    }
                }
            }
        }
    };
    #[test]
    fn test_xavetar_method_from_start_era() {
        let (mut days, mut month, mut year): (u16, u8, u64);

        for (view, shift) in [
            (CalendarView::Solar, SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_SOLAR),
            (CalendarView::Julian, SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_JULIAN),
            (CalendarView::Gregorian, SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_GREGORIAN)
        ] {
            let era_days_to_test: u128 = 10_000_000_u128;

            for era_day in 1_u128..=era_days_to_test {
                (year, days) = year_from_presentation_days(view, era_day);

                month = month_from_days(view, year, &mut days).index();

                assert_eq!(<Date as Xavetar>::from(view, year, month, days as u8).index(), Week::from(shift).next_nth(era_day).index());
            }
        }
    }

    #[test]
    fn test_xavetar_method_to_end_era() {
        let (mut days, mut month, mut year): (u16, u8, u64);

        for (view, shift) in [
            (CalendarView::Solar, SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_SOLAR),
            (CalendarView::Julian, SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_JULIAN),
            (CalendarView::Gregorian, SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_GREGORIAN)
        ] {
            let (era_days_to_test, max_type_era_day): (u128, u128)
            =
            (
                10_000_000_u128,
                days_from_presentation_date(view, u64::MAX, December.index(), December.days(is_leap_year(view, u64::MAX)))
            );

            for era_day in max_type_era_day-era_days_to_test..=max_type_era_day {
                (year, days) = year_from_presentation_days(view, era_day);

                month = month_from_days(view, year, &mut days).index();

                assert_eq!(<Date as Xavetar>::from(view, year, month, days as u8).index(), Week::from(shift).next_nth(era_day).index());
            }
        }
    }
}