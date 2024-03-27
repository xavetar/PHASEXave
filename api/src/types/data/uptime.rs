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
    planets::{
        earth::{
            calendar::{
                constants::{
                    seconds::{
                        SECONDS_IN_DAY,
                        SECONDS_IN_HOUR,
                        SECONDS_IN_MINUTE,
                        SECONDS_IN_WEEK
                    }
                },
            }
        }
    }
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uptime {
    pub weeks: u128,
    pub days: u8,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8
}

impl Uptime {
    pub const fn to_seconds(&self) -> u128 {
        return (SECONDS_IN_WEEK * self.weeks) +
               (SECONDS_IN_DAY * self.days as u128) +
               (SECONDS_IN_HOUR * self.hours as u128) +
               (SECONDS_IN_MINUTE * self.minutes as u128) +
               self.seconds as u128;
    }

    pub(crate) const fn from_seconds(seconds: u128) -> Uptime {
        return Uptime {
            weeks: seconds / SECONDS_IN_WEEK,
            days: ((seconds % SECONDS_IN_WEEK) / SECONDS_IN_DAY) as u8,
            hours: ((seconds % SECONDS_IN_DAY) / SECONDS_IN_HOUR) as u8,
            minutes: ((seconds % SECONDS_IN_HOUR) / SECONDS_IN_MINUTE) as u8,
            seconds: (seconds % SECONDS_IN_MINUTE) as u8
        }
    }
}