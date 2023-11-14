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
    data::zone::{Sign, Zone},
    planets::earth::calendar::{
        constants::{
            seconds::{SECONDS_IN_MINUTE, SECONDS_IN_HOUR},
        },
    }
};

use winapi::um::timezoneapi::{GetTimeZoneInformation, TIME_ZONE_INFORMATION};

pub fn local_timezone() -> Zone {
    let mut tz_info: TIME_ZONE_INFORMATION = unsafe { std::mem::zeroed::<TIME_ZONE_INFORMATION>() };

    let result: u32 = unsafe { GetTimeZoneInformation(&mut tz_info) };

    if result > 2 {
        panic!("[ERROR]: Could not get time zone information or this result code was not in the documentation on the compilation date!");
    }

    // Microsoft this is your alternative-logic:
    //  1) Timezone <= 0 is Unsigned timezone [+1, +14].
    //  2) Timezone > 0 is Signed/Negative timezone [-1, -12].
    // This is perfect alternative-logic. Yeah, some like a shit! "Thank you" for spent my time.
    if tz_info.Bias > 0 {
        zone.sign = Sign::Signed;
    } else {
        zone.sign = Sign::Unsigned;
    }

    let mut zone: Zone = Zone::default();
    let tz_seconds: u32 = tz_info.Bias.unsigned_abs() * SECONDS_IN_MINUTE as u32;

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