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

const START_YEAR: u16 = 1970;
const BASE_DAYS_YEAR: u16 = 365;
const LEAP_DAYS_YEAR: u16 = BASE_DAYS_YEAR + 1;

const UNIX_DAYS_BEFORE_EPOCH: u128 = 719162;

const BASE_MONTH_DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

const LEAP_MONTH_DAYS: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

const fn is_leap_year_gregorian(year: u128) -> bool {
    return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
}

fn year_from_days(days: &mut u128) -> u128 {
    let mut year: u128 = START_YEAR as u128;

    loop {
        if (*days >= (BASE_DAYS_YEAR as u128)) && (*days != 0) {
            if is_leap_year_gregorian(year) {
                *days -= LEAP_DAYS_YEAR as u128;
            } else {
                *days -= BASE_DAYS_YEAR as u128;
            }
            year += 1;
        } else {
            break;
        }
    };

    return year;
}

pub fn month_from_days(year: u128, days: &mut u128) -> u8 {
    let months: &[u8; 12];

    if !is_leap_year_gregorian(year) {
        months = &BASE_MONTH_DAYS
    } else {
        months = &LEAP_MONTH_DAYS
    }

    let mut month: usize = 0;

    loop {
        if *days >= (months[month] as u128) {
            *days -= months[month] as u128;
            month += 1;
        } else {
            *days += 1;
            break;
        }
    }

    return (month + 1) as u8;
}

fn get_date(epoch_days: u128) -> (u128, u8, u128) {
    let mut day: u128 = epoch_days;

    let year: u128 = year_from_days(&mut day);
    let month: u8 = month_from_days(year, &mut day);

    return (day, month, year);
}

const fn time(unix_time: u128) -> (u8, u8, u8) {
    let hours: u8 = ((((unix_time % 86400) / 60) / 60)) as u8;
    let minutes: u8 = ((unix_time % 3600) / 60) as u8;
    let seconds: u8 = (unix_time % 60) as u8;

    return (hours, minutes, seconds);
}

const fn week_day(epoch_days: u128) -> &'static str {
    match ((UNIX_DAYS_BEFORE_EPOCH + epoch_days) % 7) as u8 + 1 {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => panic!("Invalid week day!")
    }
}

fn main() {
    let timezone: i8 = 3 as i8;

    let unix_time: u128 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Error calling SystemTime::now()").as_secs() as u128 + (timezone as u128 * 60 * 60);

    let epoch_days: u128 = (((unix_time / 60) / 60) / 24) as u128;

    let start: std::time::Instant = std::time::Instant::now();

    let (current_day, current_month, current_year) = get_date(epoch_days);

    let elapsed: core::time::Duration = start.elapsed();

    let nanoseconds: u128 = elapsed.as_nanos();
    let milliseconds: u128 = elapsed.as_millis();
    let seconds: u64 = elapsed.as_secs();

    println!("Speed: {} nanoseconds/{} milliseconds/{} seconds", nanoseconds, milliseconds, seconds);

    println!("Current date: {}. Week day: {}.", format!("{}/{}/{}", current_day, current_month, current_year), week_day(epoch_days));

    let (hours, minutes, seconds) = time(unix_time);

    println!("Current time: {:02}:{:02}:{:02}", hours, minutes, seconds);
}