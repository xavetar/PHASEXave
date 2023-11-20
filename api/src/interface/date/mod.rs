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

pub use crate::types::data::date::{Date};

use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::{
    data::zone::{Sign, Zone},
    planets::earth::calendar::{
        view::{CalendarView},
        constants::{
            months::{Months},
        },
    },
    counter::unix_time::{
        constants::{
            days::{
                UNIX_DAYS_BEFORE_EPOCH_JULIAN,
                UNIX_DAYS_BEFORE_EPOCH_GREGORIAN
            }
        },
        functions::{
            year_from_days, month_from_days,
            epoch_days_from_seconds
        }
    },
};

#[cfg(any(
    feature = "platform_specific_functions_darwin",
    feature = "platform_specific_functions_unix",
    feature = "platform_specific_functions_windows"
))]
use crate::platform::tz::{local_timezone};
 
impl Date {
    pub fn utc(view: CalendarView) -> Date {
        let unix_time: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error calling SystemTime::now().duration_since(UNIX_EPOCH)").as_secs() as u128;

        return Self::to_date(
            view,
            unix_time,
            Zone {
                sign: Sign::Unsigned,
                hours: 0, minutes: 0,
                seconds: 0
            },
            true
            );
    }

    pub fn now(view: CalendarView, timezone: Zone) -> Date {
        let unix_time: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error calling SystemTime::now().duration_since(UNIX_EPOCH)").as_secs() as u128;

        return Self::to_date(view, unix_time, timezone, false);
    }

    #[cfg(any(
        feature = "platform_specific_functions_darwin",
        feature = "platform_specific_functions_unix",
        feature = "platform_specific_functions_windows"
    ))]
    pub fn local(view: CalendarView) -> Date {
        let unix_time: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error calling SystemTime::now().duration_since(UNIX_EPOCH)").as_secs() as u128;

        return Self::to_date(view, unix_time, local_timezone(), false);
    }

    pub fn from(view: CalendarView, unix_time: u128, timezone: Zone, tz_in_unixtime: bool) -> Date {
        return Self::to_date(view, unix_time, timezone, tz_in_unixtime);
    }

    fn to_date(view: CalendarView, mut unix_time: u128, timezone: Zone, tz_in_unixtime: bool) -> Date {
        if !tz_in_unixtime {
            let timezone_seconds: u128 = timezone.to_seconds();

            if unix_time < timezone_seconds && timezone.sign == Sign::Signed {
                panic!("[ERROR]: Overflow, signed timezone override unix_time!")
            }

            if timezone.sign == Sign::Signed {
                unix_time -= timezone_seconds;
            } else if timezone.sign == Sign::Unsigned {
                unix_time += timezone_seconds;
            }
        }

        let mut days: u128;
        let mut date: Date = Date::default();

        (date.era_days, _) = epoch_days_from_seconds(unix_time);

        if view == CalendarView::Julian {
            date.era_days += UNIX_DAYS_BEFORE_EPOCH_JULIAN;
        } else if view == CalendarView::Gregorian {
            date.era_days += UNIX_DAYS_BEFORE_EPOCH_GREGORIAN;
        } else {
            panic!("[ERROR]: Unknown CalendarView (to_date)!")
        }

        // Текущий день, который идёт, фактически не является днём эры, он ещё не закончился, но визуально это +1
        date.era_days += 1;

        (date.year, days) = year_from_days(view, date.era_days);
        date.month = month_from_days(view, date.year, &mut days).index();
        (date.day, date.timezone, date.unix_time, date.view) = (days as u8, timezone, unix_time, view);

        return date;
    }

    pub fn month(&self) -> Months {
        return Months::from(self.month);
    }
}