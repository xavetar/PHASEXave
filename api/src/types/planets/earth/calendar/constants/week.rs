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

/*
    Названия констант были измененны с:

    SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR => SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_SOLAR
    SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN => SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_JULIAN
    SHIFT_BEFORE_FIRST_WEEK_DAY_GREGORIAN => SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_GREGORIAN

    Почему? Потому-что представление календаря, может содержать в себе дни с BCE (Before Common Era),
    использование предыдущих названий было бы не совсем корректным и запутывающим, ведь использование
    таких наименований подразумевает, что весь календарь принадлежит к текущей эре и вызывает побочные
    ложные умозаключения.

    Дата представления, дата смещения от представления, не тоже самое, что дата в текущей эре,
    дата представления может содержать в себе дни или смещения от дней другой эры. Она может быть
    привязана к дням другой эры, для того чтобы сохранить когерентность календарной системы и её
    совместимость с BCE (Before Common Era). Абстрактный год, содержит в себе дни и предыдущей эры
    и текущей.
*/

pub const DAYS_IN_WEEK: u8 = 7_u8;
pub const REPEAT_WEAK_DAY_CYCLE: u8 = DAYS_IN_WEEK;

pub const SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_SOLAR: u8 = 0_u8;
pub const SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_JULIAN: u8 = 5_u8;
pub const SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_GREGORIAN: u8 = 0_u8;

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