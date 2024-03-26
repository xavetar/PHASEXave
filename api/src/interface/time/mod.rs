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

pub use crate::types::data::time::{Time};

use crate::types::{
    data::{
        zone::{Sign, Zone}
    }
};

#[cfg(any(
    feature = "platform_specific_functions_darwin",
    feature = "platform_specific_functions_unix",
    feature = "platform_specific_functions_windows"
))]
use crate::platform::{
    tz::{local_timezone},
};

impl Time {
    pub fn utc() -> Time {
        return Self::of(
            Self::unix().as_secs() as u128,
            Zone {
                sign: Sign::Unsigned,
                hours: 0, minutes: 0,
                seconds: 0
            },
            true
        );
    }

    pub fn now(time_zone: Zone) -> Time {
        return Self::of(Self::unix().as_secs() as u128, time_zone, false);
    }

    #[cfg(any(
        feature = "platform_specific_functions_darwin",
        feature = "platform_specific_functions_unix",
        feature = "platform_specific_functions_windows"
    ))]
    pub fn local() -> Time {
        return Self::of(Self::unix().as_secs() as u128, local_timezone(), false);
    }

    pub fn from(unix: u128, time_zone: Zone, zone_in_unix: bool) -> Time {
        return Self::of(unix, time_zone, zone_in_unix);
    }
}

#[cfg(test)]
mod tests {

    use super::{
        Time,
        Sign, Zone,
        local_timezone
    };

    use libc::{
        time_t, tm,
        gmtime_r, localtime_r
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

        let gmt_time_zone: Zone = Zone { sign: Sign::Unsigned, hours: 0_u8, minutes: 0_u8, seconds: 0_u8 };

        let current_seconds: u64 = Time::now(gmt_time_zone).unix_time as u64;

        for unix_time in (0..=current_seconds).step_by(SECONDS_IN_DAY as usize) {
            let time_c: time_t = unix_time as time_t;

            if unsafe { gmtime_r(&time_c, &mut time_struct_libc) } == std::ptr::null_mut() {
                panic!("[ERROR]: Pointer is NULL (gmtime_r)!")
            }

            let time: Time = Time::from(unix_time as u128, gmt_time_zone, false);

            assert_eq!(
                (
                    time_struct_libc.tm_hour as u8,
                    time_struct_libc.tm_min as u8,
                    time_struct_libc.tm_sec as u8,
                    time_struct_libc.tm_gmtoff.unsigned_abs() as u32
                ),
                (
                    time.hours,
                    time.minutes,
                    time.seconds,
                    time.time_zone.to_seconds()
                )
            )
        }
    }

    #[test]
    #[cfg(any(
        feature = "platform_specific_functions_darwin",
        feature = "platform_specific_functions_unix",
        feature = "platform_specific_functions_windows"
    ))]
    fn test_fuzz_time_with_libc() {
        let (mut time, mut sign): (Time, Sign);

        let mut time_struct_libc: tm = unsafe { std::mem::zeroed::<tm>() };

        let current_seconds: u64 = Time::now(local_timezone()).unix_time as u64;

        for unix_time in (0..=current_seconds).step_by(SECONDS_IN_DAY as usize) {
            let time_c: time_t = unix_time as time_t;

            if unsafe { localtime_r(&time_c, &mut time_struct_libc) } == std::ptr::null_mut() {
                panic!("[ERROR]: Pointer is NULL (localtime_r)!")
            }

            if time_struct_libc.tm_gmtoff < 0 { sign = Sign::Signed } else { sign = Sign::Unsigned };

            time = Time::from(unix_time as u128, Zone::from_seconds(sign, time_struct_libc.tm_gmtoff.unsigned_abs() as u32), false);

            assert_eq!(
                (
                    time_struct_libc.tm_hour as u8,
                    time_struct_libc.tm_min as u8,
                    time_struct_libc.tm_sec as u8,
                    time_struct_libc.tm_gmtoff.unsigned_abs() as u32
                ),
                (
                    time.hours,
                    time.minutes,
                    time.seconds,
                    time.time_zone.to_seconds()
                )
            )
        }
    }
}