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

use std::{
    time::{
        Duration,
        SystemTime,
        UNIX_EPOCH
    }
};

use super::{
    zone::{
        Sign, Zone
    }
};

use crate::types::{
    planets::{
        earth::{
            calendar::{
                constants::{
                    seconds::{
                        SECONDS_IN_DAY,
                        SECONDS_IN_HOUR,
                        SECONDS_IN_MINUTE
                    }
                },
            }
        }
    }
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub time_zone: Zone,
    pub unix_time: u128
}

impl Time {
    pub(crate) fn unix() -> Duration {
        return SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error calling SystemTime::now().duration_since(UNIX_EPOCH)");
    }

    pub(crate) fn of(mut unix_time: u128, time_zone: Zone, zone_in_unix: bool) -> Time {
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

        return Time::from_seconds(unix_time, time_zone);
    }

    pub(crate) const fn from_seconds(unix: u128, zone: Zone) -> Time {
        return Time {
            hours: ((unix % SECONDS_IN_DAY) / SECONDS_IN_HOUR) as u8,
            minutes: ((unix % SECONDS_IN_HOUR) / SECONDS_IN_MINUTE) as u8,
            seconds: (unix % SECONDS_IN_MINUTE) as u8,
            time_zone: zone,
            unix_time: unix
        }
    }
}
