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
    CalendarView, Months, Week,
    constants::{DAYS_IN_WEEK, MONTHS_IN_YEAR},
};

const MAX_DAYS_IN_MONTH: u8 = 31;

const MAX_SHIFT_DAYS_IN_WEEK: u8 = DAYS_IN_WEEK - 1;

const COLUMNS_MATRIX_MONTH: u8 = DAYS_IN_WEEK;
// Equivalent to: (MAX_SHIFT_DAYS_IN_WEEK + MAX_DAYS_IN_MONTH).div_ceil(DAYS_IN_WEEK)
const ROWS_MATRIX_MONTH: u8 = (MAX_SHIFT_DAYS_IN_WEEK + MAX_DAYS_IN_MONTH + (DAYS_IN_WEEK - 1)) / DAYS_IN_WEEK;

const DAYS_IN_MONTH_LINEAR: u8 = COLUMNS_MATRIX_MONTH * ROWS_MATRIX_MONTH;

const DAY_WIDTH: u8 = 2;
const DAY_FIELD_WIDTH: u8 = DAY_WIDTH + 1;

const LINE_WIDTH: u8 = DAY_FIELD_WIDTH * DAYS_IN_WEEK;
const LINE_HEIGTH: u8 = ROWS_MATRIX_MONTH + 1 + 1 + 1; // rows + week names + space + header

// 2D Array
pub type DAYS_IN_MONTHS_LINEAR = [[u8; DAYS_IN_MONTH_LINEAR as usize]; MONTHS_IN_YEAR as usize];

// 2D Array
pub type MONTH_TEXT = [[char; LINE_WIDTH as usize]; (LINE_HEIGTH) as usize];
// 3D Array
pub type TEXT_BY_MONTHS_IN_YEAR = [[[char; LINE_WIDTH as usize]; (LINE_HEIGTH) as usize]; MONTHS_IN_YEAR as usize];

fn to_chars<T: Default + Copy + std::ops::IndexMut<usize, Output = char>>(cursor: &mut usize, text: String) -> T {
    let mut line: T = Default::default();

    for (i, c) in text.chars().enumerate() {
        if c != '\0' {
            line[i] = c;
        }
    }

    *cursor += 1;

    return line;
}

pub fn format_calendar_from_text_months(year: u64, columns: u8, margin: [u8; 4], text_by_months_in_year: TEXT_BY_MONTHS_IN_YEAR) -> Vec<Vec<char>> {
    let mut cursor_line: usize = 0;

    // Equivalent to: MONTHS_IN_YEAR.div_ceil(columns)
    let row_objects: u8 = (MONTHS_IN_YEAR + (columns - 1)) / columns;

    let width_calendar: u16 = (LINE_WIDTH as u16 * columns as u16) + ((margin[1] as u16 + margin[3] as u16) * (columns as u16));
    let height_calendar: u16 = (LINE_HEIGTH as u16 * row_objects as u16) + ((margin[0] as u16 + margin[2] as u16) * (row_objects as u16)) + 1 + 1; // calendar height + space + year (header)

    let mut calendar_text: Vec<Vec<char>> = Vec::<Vec<char>>::with_capacity(height_calendar as usize);

    for _ in 0..height_calendar {
        calendar_text.push(vec!['0'; width_calendar as usize]);
    }

    calendar_text[cursor_line] = format!("{:^width$}", year, width = width_calendar as usize).chars().collect::<Vec<char>>(); cursor_line += 1; // Calendar year
    calendar_text[cursor_line].clear(); cursor_line += 1; // Space

    let mut month_index: u8 = 0;

    for _ in 0..row_objects {
        // Margin top
        for _ in 0..margin[0] {
            calendar_text[cursor_line].clear(); cursor_line += 1;
        }

        if month_index < MONTHS_IN_YEAR {
            for line in 0..LINE_HEIGTH {
                calendar_text[cursor_line].clear();
                for column in 0..columns {
                    if (month_index + column) < MONTHS_IN_YEAR {
                        calendar_text[cursor_line].append(format!("{:^width$}", "", width = margin[3] as usize).chars().collect::<Vec<char>>().as_mut());
                        calendar_text[cursor_line].append(text_by_months_in_year[(month_index + column) as usize][line as usize].into_iter().collect::<Vec<char>>().as_mut());
                        calendar_text[cursor_line].append(format!("{:^width$}", "", width = margin[1] as usize).chars().collect::<Vec<char>>().as_mut());
                    }
                }
                cursor_line += 1;
            }
            month_index += columns;
        }

        // Margin Bottom
        for _ in 0..margin[2] {
            calendar_text[cursor_line].clear(); cursor_line += 1;
        }
    }

    return calendar_text;
}

pub fn format_months_to_text(calendar: &DAYS_IN_MONTHS_LINEAR) -> TEXT_BY_MONTHS_IN_YEAR {
    let mut text_by_months_in_year: TEXT_BY_MONTHS_IN_YEAR =  [[['0'; LINE_WIDTH as usize]; (LINE_HEIGTH) as usize]; MONTHS_IN_YEAR as usize];

    for (month, matrix) in calendar.iter().enumerate() {
        let mut cursor_line: usize = 0;

        let mut month_text: MONTH_TEXT = [['0'; LINE_WIDTH as usize]; (LINE_HEIGTH) as usize];

        month_text[cursor_line - 1] = to_chars::<[char; LINE_WIDTH as usize]>(&mut cursor_line, format!("{:^width$}", Months::from((month + 1) as u8).name(), width = LINE_WIDTH as usize));

        month_text[cursor_line - 1] = to_chars::<[char; LINE_WIDTH as usize]>(&mut cursor_line, format!("{:^width$}", "", width = LINE_WIDTH as usize));

        month_text[cursor_line - 1] = to_chars::<[char; LINE_WIDTH as usize]>(&mut cursor_line, format!("{:^width$}", "Mo Tu We Th Fr Sa Su", width = LINE_WIDTH as usize));

        let mut line: String = String::from("");

        for (i, field) in matrix.iter().enumerate() {
            if *field == 0 {
                line.push_str("   ");
            } else {
                line.push_str(&format!("{:2} ", field));
            }

            if ((i + 1) as u8 % DAYS_IN_WEEK) == 0 && (line.len() as u8) == LINE_WIDTH {
                month_text[cursor_line - 1] = to_chars::<[char; LINE_WIDTH as usize]>(&mut cursor_line, format!("{:^21}", line));
                line.clear();
            }
        }

        text_by_months_in_year[month] = month_text;
    }

    return text_by_months_in_year;
}

pub fn format_months_by_days(view: CalendarView, method: fn(CalendarView, u64, u8, u8) -> Week, year: u64, is_leap_year: bool) -> DAYS_IN_MONTHS_LINEAR {
    // 2D Array
    let mut days_in_months: DAYS_IN_MONTHS_LINEAR = [[0; DAYS_IN_MONTH_LINEAR as usize]; MONTHS_IN_YEAR as usize];

    for month_in_year in 1..(MONTHS_IN_YEAR + 1) {
        let month: Months = Months::from(month_in_year);
        let shift: u8 = method(view, year, month.index(), 1).index() - 1;
        for day_in_month in 1..(month.days(is_leap_year) + 1) {
            days_in_months[(month_in_year - 1) as usize][(shift + (day_in_month - 1)) as usize] = day_in_month;
        }

    }

    return days_in_months;
}