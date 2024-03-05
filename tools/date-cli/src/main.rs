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

use PHASEXave::{
    CalendarView,
    Date, Time,
    Zone, Sign,
    Months, Week,
    RataDie,
};

use date_cli::{
    parse::{parse_args},
};

fn make_output(view: CalendarView, timezone: Zone, method: fn(CalendarView, u128, u8, u8) -> Week) {
    let date: Date;

    if timezone.sign == Sign::Unsigned && timezone.hours == 255 && timezone.minutes == 255 && timezone.seconds == 255 {
        date = Date::local(view);
    } else {
        date = Date::now(view, timezone);
    }

    let time: Time = Time::now(date.timezone);

    print!(
        "{day_of_week} {month} {day} {hours:02}:{minutes:02}:{seconds:02}",
        day_of_week = method(view, date.year, date.month, date.day).name(),
        month = Months::from(date.month).name(),
        day = date.day,
        hours = time.hours,
        minutes = time.minutes,
        seconds = time.seconds
    );

    if date.timezone.sign == Sign::Unsigned {
        print!("+")
    } else if date.timezone.sign == Sign::Signed {
        print!("-");
    } else {
        panic!("[PANIC] Unknown timezone sign!");
    }

    println!(
        "{timezone_hours:02}:{timezone_minutes:02}:{timezone_seconds:02} {year} {calendar_view:?}",
        timezone_hours = date.timezone.hours,
        timezone_minutes = date.timezone.minutes,
        timezone_seconds = date.timezone.seconds,
        year = date.year,
        calendar_view = view
    )
}

fn main() {
    let (
        mut timezone,
        mut method,
        mut view,
    )
    :
    (
        Zone,
        fn(CalendarView, u128, u8, u8) -> Week,
        CalendarView,
    )
    =
    (
        Zone { sign: Sign::Unsigned, hours: 255, minutes: 255, seconds: 255 },
        <Date as RataDie>::from,
        CalendarView::Gregorian,
    );

    parse_args(&mut timezone, &mut method, &mut view);
    make_output(view, timezone, method);
}
