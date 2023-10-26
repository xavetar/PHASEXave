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
            seconds::{SECONDS_IN_DAY},
            days::{ALIGN_JULIAN_TO_GREGORIAN_DAYS}
        }
    },
    it::unix_time::{
        constants::{
            days::{UNIX_DAYS_BEFORE_EPOCH_GREGORIAN}
        },
        functions::{
            year_from_days, month_from_days, era_days_from_date
        }
    }
};

pub trait Gregorian {
    fn to_gregorian(&mut self);
}

impl Gregorian for Date {
    fn to_gregorian(&mut self) {
        match self.view {
            CalendarView::Julian => {
                let mut days: u128;

                // Вычисление дней эры (+1 находится в самой дате)
                self.era_days = era_days_from_date(self.view.clone(), self.year, self.month, self.day);

                // Удаление отсутствующих дней в Григорианском календаре
                if self.era_days >= ALIGN_JULIAN_TO_GREGORIAN_DAYS as u128 {
                    self.era_days -= ALIGN_JULIAN_TO_GREGORIAN_DAYS as u128;
                } else {
                    panic!("[IMPOSSIBLE]: This days is missing in Gregorian Calendar!")
                }

                // Внутри функции происходит неявное смещение, из-за чего использование функции excess_leap_years является излишним
                // и может привести к неточностям. Этот метод основан на подсчёте дней, он действителен для любой даты и универсален.
                (self.year, days) = year_from_days(CalendarView::Gregorian, self.era_days);

                self.month = month_from_days(CalendarView::Gregorian, self.year, &mut days).index();

                if self.era_days > UNIX_DAYS_BEFORE_EPOCH_GREGORIAN {
                    self.unix_time = ((self.era_days - (UNIX_DAYS_BEFORE_EPOCH_GREGORIAN + 1)) * SECONDS_IN_DAY) + (self.unix_time % SECONDS_IN_DAY);
                } else {
                    self.unix_time = 0;
                }

                self.day = days as u8;

                self.view = CalendarView::Gregorian;
            },
            CalendarView::Gregorian => (),
            _ => panic!("[ERROR]: Unknown CalendarView (to_julian)")
        }
    }
}