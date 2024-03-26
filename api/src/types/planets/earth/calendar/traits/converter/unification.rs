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

use super::{zone_re_calc};

use crate::types::{
    data::{
        date::{Date},
    },
    planets::{
        earth::{
            calendar::{
                view::{CalendarView},
                constants::{
                    seconds::{SECONDS_IN_DAY},
                },
            }
        }
    },
    counter::{
        unix_time::{
            functions::{
                year_from_presentation_days, month_from_days,
            }
        }
    }
};

pub trait Converter {
    fn fill_date(&mut self, to: CalendarView);
    fn fill_time(&mut self, day_before_timer_start: u128, zone_in_unix: bool);
}

impl Converter for Date {
    fn fill_date(&mut self, to: CalendarView) {
        let mut days: u16;

        (self.year, days) = year_from_presentation_days(to, self.era_days);

        self.month = month_from_days(to, self.year, &mut days).index();

        (self.day, self.view) = (days as u8, to);
    }

    fn fill_time(&mut self, day_before_timer_start: u128, zone_in_unix: bool) {
        if self.era_days > day_before_timer_start {
            let day_seconds: u128 = self.unix_time % SECONDS_IN_DAY;

            self.unix_time = (self.era_days - (day_before_timer_start + 1)) * SECONDS_IN_DAY;

            // Используется в случае когда временная зона не находится в unix time, позволяет указать время внутри дня,
            // с учётом секунд внутри дня ± часовой пояс.
            if !zone_in_unix {
                zone_re_calc(self.time_zone, &mut self.unix_time, day_seconds, &mut self.era_days);
            } else {
                self.unix_time += day_seconds;
            }
        } else {
            self.unix_time = 0_u128;
        }
    }
}