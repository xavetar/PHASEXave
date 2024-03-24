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

use super::{
    seconds::{BASE_MONTH_SECONDS, CALENDAR_LEAP_SECONDS_IN_LEAP_YEAR}
};

pub const MONTHS_IN_YEAR: u8 = 12_u8;

pub const BASE_MONTH_DAYS: [u8; MONTHS_IN_YEAR as usize] = [31_u8, 28_u8, 31_u8, 30_u8, 31_u8, 30_u8, 31_u8, 31_u8, 30_u8, 31_u8, 30_u8, 31_u8];
pub const LEAP_MONTH_DAYS: [u8; MONTHS_IN_YEAR as usize] = [31_u8, 29_u8, 31_u8, 30_u8, 31_u8, 30_u8, 31_u8, 31_u8, 30_u8, 31_u8, 30_u8, 31_u8];

pub enum Months {
    January   = 1,
    February  = 2,
    March     = 3,
    April     = 4,
    May       = 5,
    June      = 6,
    July      = 7,
    August    = 8,
    September = 9,
    October   = 10,
    November  = 11,
    December  = 12
}

impl Months {
    pub const fn index(&self) -> u8 {
        match self {
            Months::January   => 1_u8,
            Months::February  => 2_u8,
            Months::March     => 3_u8,
            Months::April     => 4_u8,
            Months::May       => 5_u8,
            Months::June      => 6_u8,
            Months::July      => 7_u8,
            Months::August    => 8_u8,
            Months::September => 9_u8,
            Months::October   => 10_u8,
            Months::November  => 11_u8,
            Months::December  => 12_u8,
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            Months::January   => "January",
            Months::February  => "February",
            Months::March     => "March",
            Months::April     => "April",
            Months::May       => "May",
            Months::June      => "June",
            Months::July      => "July",
            Months::August    => "August",
            Months::September => "September",
            Months::October   => "October",
            Months::November  => "November",
            Months::December  => "December"
        }
    }

    pub const fn days(&self, leap: bool) -> u8 {
        match self {
            Months::January   => BASE_MONTH_DAYS[0],
            Months::February  => {
                if !leap {
                    return BASE_MONTH_DAYS[1];
                } else {
                    return BASE_MONTH_DAYS[1] + 1_u8;
                }
            },
            Months::March     => BASE_MONTH_DAYS[2],
            Months::April     => BASE_MONTH_DAYS[3],
            Months::May       => BASE_MONTH_DAYS[4],
            Months::June      => BASE_MONTH_DAYS[5],
            Months::July      => BASE_MONTH_DAYS[6],
            Months::August    => BASE_MONTH_DAYS[7],
            Months::September => BASE_MONTH_DAYS[8],
            Months::October   => BASE_MONTH_DAYS[9],
            Months::November  => BASE_MONTH_DAYS[10],
            Months::December  => BASE_MONTH_DAYS[11],
        }
    }

    pub const fn seconds(&self, leap: bool) -> u128 {
        match self {
            Months::January => BASE_MONTH_SECONDS[0],
            Months::February => {
                if !leap {
                    return BASE_MONTH_SECONDS[1];
                } else {
                    return BASE_MONTH_SECONDS[1] + CALENDAR_LEAP_SECONDS_IN_LEAP_YEAR;
                }
            },
            Months::March     => BASE_MONTH_SECONDS[2],
            Months::April     => BASE_MONTH_SECONDS[3],
            Months::May       => BASE_MONTH_SECONDS[4],
            Months::June      => BASE_MONTH_SECONDS[5],
            Months::July      => BASE_MONTH_SECONDS[6],
            Months::August    => BASE_MONTH_SECONDS[7],
            Months::September => BASE_MONTH_SECONDS[8],
            Months::October   => BASE_MONTH_SECONDS[9],
            Months::November  => BASE_MONTH_SECONDS[10],
            Months::December  => BASE_MONTH_SECONDS[11],
        }
    }

    pub fn from(month: u8) -> Months {
        match month {
            1_u8         => Months::January,
            2_u8         => Months::February,
            3_u8         => Months::March,
            4_u8         => Months::April,
            5_u8         => Months::May,
            6_u8         => Months::June,
            7_u8         => Months::July,
            8_u8         => Months::August,
            9_u8         => Months::September,
            10_u8        => Months::October,
            11_u8        => Months::November,
            12_u8 | 0_u8 => Months::December,
            _  => panic!("Invalid month: {}", month),
        }
    }

    pub fn next(&self) -> Months {
        return Months::from((self.index() + 1_u8) % MONTHS_IN_YEAR);
    }

    pub fn previous(&self) -> Months {
        return Months::from((self.index() - 1_u8) % MONTHS_IN_YEAR);
    }
}