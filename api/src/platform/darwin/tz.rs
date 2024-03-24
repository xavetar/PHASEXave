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
                    seconds::{SECONDS_IN_MINUTE, SECONDS_IN_HOUR},
                },
            }
        }
    }
};

use core_foundation::{
    date::{CFAbsoluteTimeGetCurrent},
    timezone::{CFTimeZoneGetSecondsFromGMT, CFTimeZoneCopySystem}
};

pub fn local_timezone() -> Zone {
    let mut zone: Zone = Zone::default();
    let seconds: i32 = unsafe { CFTimeZoneGetSecondsFromGMT(CFTimeZoneCopySystem(), CFAbsoluteTimeGetCurrent()) } as i32;

    if seconds < 0 {
        zone.sign = Sign::Signed;
    } else {
        zone.sign = Sign::Unsigned;
    }

    let tz_seconds: u32 = seconds.unsigned_abs();

    (
        zone.hours,
        zone.minutes,
        zone.seconds
    ) = (
        (tz_seconds / (SECONDS_IN_HOUR as u32)) as u8,
        ((tz_seconds % (SECONDS_IN_HOUR  as u32)) / (SECONDS_IN_MINUTE as u32)) as u8,
        (tz_seconds % (SECONDS_IN_MINUTE as u32)) as u8
    );

    return zone;
}