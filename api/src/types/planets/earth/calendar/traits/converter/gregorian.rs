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

use super::{zone_recalc};

use crate::types::{
    data::{
        date::{Date},
    },
    planets::earth::calendar::{
        view::{CalendarView},
        constants::{
            seconds::{SECONDS_IN_DAY},
            days::{ALIGN_JULIAN_TO_CONVERT_DAYS}
        },
        functions::{
            era_days_from_date
        }
    },
    counter::unix_time::{
        constants::{
            days::{UNIX_DAYS_BEFORE_EPOCH_GREGORIAN}
        },
        functions::{
            year_from_era_days, month_from_days
        }
    }
};

pub trait Gregorian {
    fn to_gregorian(&mut self, tz_in_unixtime: bool);
}

impl Gregorian for Date {
    fn to_gregorian(&mut self, tz_in_unixtime: bool) {
        let mut days: u128;

        // Вычисление дней эры (+1 находится в самой дате)
        self.era_days = era_days_from_date(self.view.clone(), self.year, self.month, self.day);

        match self.view {
            CalendarView::Julian => {
                if self.era_days >= ALIGN_JULIAN_TO_CONVERT_DAYS as u128 {
                    self.era_days -= ALIGN_JULIAN_TO_CONVERT_DAYS as u128;
                } else {
                    panic!("[IMPOSSIBLE]: This days is missing in Gregorian Calendar! (to_gregorian)")
                }
            },
            CalendarView::Gregorian => (),
            CalendarView::Solar => (),
            _ => panic!("[ERROR]: Unknown CalendarView (to_gregorian)")
        }

        if self.era_days > UNIX_DAYS_BEFORE_EPOCH_GREGORIAN {
            let day_seconds: u128 = self.unix_time % SECONDS_IN_DAY;

            self.unix_time = (self.era_days - (UNIX_DAYS_BEFORE_EPOCH_GREGORIAN + 1)) * SECONDS_IN_DAY;

            // Используется в случае когда временная зона не находится в unix time, позволяет указать время внутри дня,
            // с учётом секунд внутри дня ± часовой пояс.
            if !tz_in_unixtime {
                zone_recalc(self.timezone, &mut self.unix_time, day_seconds, &mut self.era_days);
            } else {
                self.unix_time += day_seconds;
            }
        } else {
            self.unix_time = 0_u128;
        }

        (self.year, days) = year_from_era_days(CalendarView::Gregorian, self.era_days);

        self.month = month_from_days(CalendarView::Gregorian, self.year, &mut days).index();

        self.day = days as u8;

        self.view = CalendarView::Gregorian;
    }
}