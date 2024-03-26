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
    zone::{
        Sign, Zone
    }
};

use crate::types::{
    planets::{
        earth::{
            calendar::{
                view::{CalendarView},
                constants::{
                    seconds::{SECONDS_IN_DAY},
                    days::{JULIAN_BCE_DAYS_FIRST_YEAR},
                }
            }
        },
    },
    counter::{
        unix_time::{
            constants::{
                days::{UNIX_TIME_START_AFTER_DAY}
            },
            functions::{
                year_from_presentation_days, month_from_days,
            },
        }
    },
};

type UNIX_EPOCH = (u32, u128);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u64,
    pub time_zone: Zone,
    pub unix_time: u128,
    pub era_days: u128,
    pub view: CalendarView
}

impl Date {
    pub(crate) fn of(view: CalendarView, mut unix_time: u128, time_zone: Zone, zone_in_unix: bool) -> Date {
        if !zone_in_unix {
            let time_zone_seconds: u128 = time_zone.to_seconds() as u128;

            if unix_time < time_zone_seconds && time_zone.sign == Sign::Signed {
                panic!("[OVERFLOW]: Overflow type, unix time - time zone < zero!")
            } else if unix_time > u128::MAX - time_zone_seconds && time_zone.sign == Sign::Unsigned {
                panic!("[OVERFLOW]: Overflow type, unix time + time zone > type!")
            }

            if time_zone.sign == Sign::Signed {
                unix_time -= time_zone_seconds;
            } else if time_zone.sign == Sign::Unsigned {
                unix_time += time_zone_seconds;
            }
        }

        let mut days: u16;
        let mut date: Date = Date::default();

        let (_day_seconds, mut presentation_days, ): UNIX_EPOCH
        =
        ((unix_time % SECONDS_IN_DAY) as u32, (unix_time - (unix_time % SECONDS_IN_DAY)) / SECONDS_IN_DAY);

        presentation_days += UNIX_TIME_START_AFTER_DAY + 1_u128;

        if view == CalendarView::Julian {
            presentation_days += JULIAN_BCE_DAYS_FIRST_YEAR;
        }

        (date.year, days) = year_from_presentation_days(view, presentation_days);
        date.month = month_from_days(view, date.year, &mut days).index();

        if view == CalendarView::Julian {
            presentation_days -= JULIAN_BCE_DAYS_FIRST_YEAR;
        }

        (date.day, date.time_zone, date.unix_time, date.era_days, date.view)
        =
        (days as u8, time_zone, unix_time, presentation_days, view);

        return date;
    }
}