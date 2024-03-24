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

use PHASEXave::{
    CalendarView, Date, Week,
    RataDie,
    functions::{is_leap_year}
};

use calendar_cli::{
    text::{
        format_months_by_days,
        format_months_to_text,
        format_calendar_from_text_months
    },
    save::{save_to_file},
    parse::{parse_args},
    types::{
        enums::{Modes}
    }
};

fn make_calendar(view: CalendarView, method: fn(CalendarView, u64, u8, u8) -> Week, year: u64, columns: u8, margin: [u8; 4]) -> Vec<Vec<char>> {
    return format_calendar_from_text_months(
        year, columns, margin,
        format_months_to_text(
            &format_months_by_days(
                view, method, year, is_leap_year(view, year)
            )
        )
    );
}

fn main() {
    let (
        mut year,
        mut method,
        mut columns,
        mut margin,
        mut view,
        mut mode,
        mut filename
    )
    :
    (
        u64,
        fn(CalendarView, u64, u8, u8) -> Week,
        u8,
        [u8; 4],
        CalendarView,
        Modes,
        String
    )
    =
    (
        Date::local(CalendarView::Gregorian).year,
        <Date as RataDie>::from,
        3,
        [0, 1, 1, 1],
        CalendarView::Gregorian,
        Modes::Console,
        String::from("Calendar.txt")
    );
    
    parse_args(&mut year, &mut method, &mut columns, &mut margin, &mut view, &mut mode, &mut filename);

    let calendar_text: Vec<Vec<char>> = make_calendar(view, method, year, columns, margin);

    if mode == Modes::File {
        save_to_file(filename, calendar_text);
    } else if mode == Modes::Console {
        for month in calendar_text {
            println!("{}", String::from_iter(month));
        }
    } else {
        panic!("[ERROR]: Unknown mode!")
    }
}
