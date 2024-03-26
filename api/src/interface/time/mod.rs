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

pub use crate::types::data::time::{Time, Uptime};

use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::{
    data::{
        zone::{Sign, Zone}
    },
    planets::{
        earth::{
            calendar::{
                constants::{
                    seconds::{SECONDS_IN_MINUTE, SECONDS_IN_HOUR, SECONDS_IN_DAY}
                }
            }
        }
    }
};

#[cfg(any(
    feature = "platform_specific_functions_darwin",
    feature = "platform_specific_functions_unix",
    feature = "platform_specific_functions_windows"
))]
use crate::platform::{
    time::{uptime},
    tz::{local_timezone},
};

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

    #[cfg(any(
        feature = "platform_specific_functions_darwin",
        feature = "platform_specific_functions_unix",
        feature = "platform_specific_functions_windows"
    ))]
    pub fn local() -> Time {
        let unix_time: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error calling SystemTime::now().duration_since(UNIX_EPOCH)").as_secs() as u128;

        return Self::to_time(unix_time, local_timezone(), false);
    }

    pub fn from(unix_time: u128, timezone: Zone, timezone_in_unix_time: bool) -> Time {
        return Self::to_time(unix_time, timezone, timezone_in_unix_time);
    }

    fn to_time(mut unix_time: u128, timezone: Zone, timezone_in_unix_time: bool) -> Time {
        if !timezone_in_unix_time {
            let timezone_seconds: u128 = timezone.to_seconds() as u128;

            if unix_time < timezone_seconds && timezone.sign == Sign::Signed {
                panic!("[OVERFLOW]: Overflow type, unix time - time zone < zero!")
            } else if unix_time > u128::MAX - timezone_seconds && timezone.sign == Sign::Unsigned {
                panic!("[OVERFLOW]: Overflow type, unix time + time zone > type!")
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

#[cfg(test)]
mod tests {

    use super::{
        Time,
        Sign, Zone
    };

    use libc::{
        time_t, tm,
        gmtime_r
    };

    use crate::{
        types::{
            planets::{
                earth::{
                    calendar::{
                        constants::{
                            seconds::{SECONDS_IN_DAY}
                        }
                    }
                }
            }
        }
    };

    #[test]
    fn test_gmt_time_from_libc() {
        let mut time_struct_libc: tm = unsafe { std::mem::zeroed::<tm>() };

        let gmt_timezone: Zone = Zone { sign: Sign::Unsigned, hours: 0_u8, minutes: 0_u8, seconds: 0_u8 };

        let current_seconds: u64 = Time::now(gmt_timezone).unix_time as u64;

        for unix_time in (0..=current_seconds).step_by(SECONDS_IN_DAY as usize) {
            let time_c: time_t = unix_time as time_t;

            if unsafe { gmtime_r(&time_c, &mut time_struct_libc) } == std::ptr::null_mut() {
                panic!("[ERROR]: Pointer is NULL (gmtime_r)!")
            }

            let time: Time = Time::from(unix_time as u128, gmt_timezone, false);

            assert_eq!(
                (
                    (time_struct_libc.tm_hour) as u8,
                    (time_struct_libc.tm_min) as u8,
                    (time_struct_libc.tm_sec) as u8,
                    (time_struct_libc.tm_gmtoff) as u64
                ),
                (
                    time.hours,
                    time.minutes,
                    time.seconds,
                    time.timezone.to_seconds() as u64
                )
            )
        }
    }
}