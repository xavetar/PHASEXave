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
    data::date::{Date},
    planets::earth::calendar::{
        view::{CalendarView},
        constants::{
            week::{
                SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR,
                SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN,
                SHIFT_BEFORE_FIRST_WEEK_DAY_GREGORIAN,
                REPEAT_WEAK_DAY_CYCLE, Week,
            },
        },
        functions::{era_days_from_date}
    }
};

pub trait RataDie {
    fn week_day(&self) -> Week;
    fn from(view: CalendarView, year: u128, month: u8, day: u8) -> Week;
}

impl RataDie for Date {
    fn week_day(&self) -> Week {
        if self.era_days != 0_u128 {
            let SHIFT_BEFORE_FIRST_WEEK_DAY: u128 = match self.view {
                CalendarView::Solar => SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u128,
                CalendarView::Julian => SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u128,
                CalendarView::Gregorian => SHIFT_BEFORE_FIRST_WEEK_DAY_GREGORIAN as u128,
                _ => panic!("[ERROR]: Unknown CalendarView (RataDie - week_day).")
            };

            return Week::from(((self.era_days + SHIFT_BEFORE_FIRST_WEEK_DAY) % (REPEAT_WEAK_DAY_CYCLE as u128)) as u8);
        } else {
            return <Date as RataDie>::from(self.view.clone(), self.year, self.month, self.day);
        }
    }

    fn from(view: CalendarView, year: u128, month: u8, day: u8) -> Week {
        let SHIFT_BEFORE_FIRST_WEEK_DAY: u128 = match view {
            CalendarView::Solar => SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u128,
            CalendarView::Julian => SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u128,
            CalendarView::Gregorian => SHIFT_BEFORE_FIRST_WEEK_DAY_GREGORIAN as u128,
            _ => panic!("[ERROR]: Unknown CalendarView (RataDie - from).")
        };

        let era_days: u128 = era_days_from_date(view, year, month, day);

        return Week::from(((era_days + SHIFT_BEFORE_FIRST_WEEK_DAY) % (REPEAT_WEAK_DAY_CYCLE as u128)) as u8);
    }
}