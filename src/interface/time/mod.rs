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

use std::time::{SystemTime, UNIX_EPOCH};
pub use crate::types::data::time::{Time, Uptime};
use crate::types::{
    data::zone::{Sign, Zone},
    planets::earth::calendar::{
        constants::{
            seconds::{SECONDS_IN_MINUTE, SECONDS_IN_HOUR, SECONDS_IN_DAY}
        }
    }
};

#[cfg(any(
    feature = "platform_specific_functions_darwin",
    feature = "platform_specific_functions_unix",
    feature = "platform_specific_functions_windows"
))]
use crate::platform::time::{uptime};

impl Time {
    pub fn utc() -> Time {
        let unix_time: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error calling SystemTime::now().duration_since(UNIX_EPOCH)").as_secs() as u128;

        return Self::to_time(
            unix_time,
            Zone {
                sign: Sign::Unsigned,
                hours: 0, minutes: 0,
                seconds: 0
            },
                true
            );
    }

    pub fn now(timezone: Zone) -> Time {
        let unix_time: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error calling SystemTime::now().duration_since(UNIX_EPOCH)").as_secs() as u128;

        return Self::to_time(unix_time, timezone, false);
    }

    pub fn from(unix_time: u128, timezone: Zone, tz_in_unixtime: bool) -> Time {
        return Self::to_time(unix_time, timezone, tz_in_unixtime);
    }

    fn to_time(mut unix_time: u128, timezone: Zone, tz_in_unixtime: bool) -> Time {
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

        return Time {
            hours: ((unix_time % SECONDS_IN_DAY) / SECONDS_IN_HOUR) as u8,
            minutes: ((unix_time % SECONDS_IN_HOUR) / SECONDS_IN_MINUTE) as u8,
            seconds: (unix_time % SECONDS_IN_MINUTE) as u8,
            timezone: timezone,
            unix_time: unix_time
        };
    }

    #[cfg(any(
        feature = "platform_specific_functions_darwin",
        feature = "platform_specific_functions_unix",
        feature = "platform_specific_functions_windows"
    ))]
    pub fn absolute() -> Uptime {
        let seconds: u128 = uptime() as u128;

        return Uptime {
            hours: (seconds / SECONDS_IN_HOUR),
            minutes: ((seconds % SECONDS_IN_HOUR) / SECONDS_IN_MINUTE) as u8,
            seconds: (seconds % SECONDS_IN_MINUTE) as u8
        }
    }
}