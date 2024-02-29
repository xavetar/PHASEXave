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

pub const DAYS_IN_WEEK: u8 = 7_u8;
pub const REPEAT_WEAK_DAY_CYCLE: u8 = DAYS_IN_WEEK;

pub const SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN: u8 = 5_u8;
pub const SHIFT_BEFORE_FIRST_WEEK_DAY_GREGORIAN: u8 = 0_u8;
pub const SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR: u8 = 0_u8;

#[derive(Debug)]
pub enum Week {
    Monday    = 1,
    Tuesday   = 2,
    Wednesday = 3,
    Thursday  = 4,
    Friday    = 5,
    Saturday  = 6,
    Sunday    = 7
}

impl Week {
    pub fn index(&self) -> u8 {
        match self {
            Week::Monday    => 1_u8,
            Week::Tuesday   => 2_u8,
            Week::Wednesday => 3_u8,
            Week::Thursday  => 4_u8,
            Week::Friday    => 5_u8,
            Week::Saturday  => 6_u8,
            Week::Sunday    => 7_u8,
        }
    }

    pub fn from(week_day: u8) -> Week {
        match week_day {
            1_u8         => Week::Monday,
            2_u8         => Week::Tuesday,
            3_u8         => Week::Wednesday,
            4_u8         => Week::Thursday,
            5_u8         => Week::Friday,
            6_u8         => Week::Saturday,
            7_u8 | 0_u8  => Week::Sunday,
            _  => panic!("Invalid week day: {}", week_day),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Week::Monday    => "Monday",
            Week::Tuesday   => "Tuesday",
            Week::Wednesday => "Wednesday",
            Week::Thursday  => "Thursday",
            Week::Friday    => "Friday",
            Week::Saturday  => "Saturday",
            Week::Sunday    => "Sunday"
        }
    }
}