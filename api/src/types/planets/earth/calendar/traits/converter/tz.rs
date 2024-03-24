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
        zone::{Sign, Zone}
    },
    planets::{
        earth::{
            calendar::{
                constants::{
                    seconds::{SECONDS_IN_DAY},
                },
            }
        }
    },
};

pub fn zone_recalc(timezone: Zone, unix_time: &mut u128, day_seconds: u128, era_days: &mut u128) {
    let tz_sec: u128 = timezone.to_seconds() as u128;
    if timezone.sign == Sign::Signed && *unix_time < tz_sec {
        panic!("[ERROR]: Overflow, signed timezone override self.unix_time!!")
    } else {
        if timezone.sign == Sign::Signed && tz_sec > 0 {
            if day_seconds >= tz_sec {
                *unix_time -= tz_sec;
                *unix_time += day_seconds;
            } else if day_seconds < tz_sec {
                // Equivalent to: (tz_sec - day_seconds).div_ceil(SECONDS_IN_DAY): (a + (b - 1)) / b
                *era_days -= ((tz_sec - day_seconds) + (SECONDS_IN_DAY - 1)) / SECONDS_IN_DAY;
                *unix_time -= tz_sec;
                *unix_time += day_seconds;
            }
        } else if timezone.sign == Sign::Unsigned {
            let total_secs: u128 = tz_sec + day_seconds;
            if total_secs < SECONDS_IN_DAY {
                *unix_time += tz_sec;
                *unix_time += day_seconds;
            } else if total_secs >= SECONDS_IN_DAY {
                // Equivalent to: total_secs.div_euclid(SECONDS_IN_DAY): (a - (a mod b)) / b
                *era_days += (total_secs - (total_secs % SECONDS_IN_DAY)) / SECONDS_IN_DAY;
                *unix_time += tz_sec;
                *unix_time += day_seconds;
            }
        }
    }
}