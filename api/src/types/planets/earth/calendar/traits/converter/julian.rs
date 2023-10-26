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
    Unix time остаётся неизменным, почему? Потому-что фактически отсчёт его начался после 31.12.1969, тогда как два
    дополнительных дня находятся исключительно в начале, т.е 1 и 2 день 1 AD (2.1.1 - Julian), как дни эры они должны быть,
    но в отсчёте unix time они не учавствуют.

    Разъяснение дополнительных деталей алгоритма:

    НЕПРАВИЛЬНО:

    31.12.1969 - это относительная дата и равна она 719177 дням эры или 719162 по Григорианскому календарю.
    Таким образом unix time абсолютно независим от календаря, только от точки начала отсчёта.

    ПРАВИЛЬНО:

    Конечная дата определяется по абсолютному количеству дней, поэтому когда дата меньше, количество дней эпохи одинаково,
    между календарями. Ошибочно делать предположение согласно которому 719177 - это действительно сумма всех дней до 31.12.1969,
    относительно Григорианского календаря, ведь эти даты не равны между Григорианским и Юлианским календарями.
    Согласно Юлианскому календарю дата отсчёта начала эпохи Unix равна 719164 (полных дня эпохи по 18.12.1969) дням или 19.12.1969
    по Юлианскому календарю или по Григорианскому -2 (отсутствующих дня), т.е 719162 (полных дня эпох по 31.12.1969) или 1.1.1970.
*/

use super::{Converter};

use crate::types::{
    data::{
        date::{Date},
    },
    planets::earth::calendar::{
        view::{CalendarView},
        constants::{
            days::{ALIGN_JULIAN}
        },
        functions::{
            days_from_date, era_days_from_date
        }
    },
    counter::unix_time::{
        constants::{
            days::{UNIX_TIME_START_AFTER_DAY}
        },
    }
};

pub trait Julian : Converter {
    fn to_date(&mut self, tz_in_unixtime: bool);
    fn to_presentation(&mut self, tz_in_unixtime: bool);
}

impl Julian for Date {
    fn to_date(&mut self, tz_in_unixtime: bool) {
        match self.view {
            CalendarView::Julian => {
                self.era_days = era_days_from_date(self.view.clone(), self.year, self.month, self.day);
            },
            CalendarView::Gregorian => {
                self.era_days = era_days_from_date(self.view.clone(), self.year, self.month, self.day) + ALIGN_JULIAN as u128;
            },
            CalendarView::Solar => {
                self.era_days = era_days_from_date(self.view.clone(), self.year, self.month, self.day) + ALIGN_JULIAN as u128;
            },
            _ => panic!("[ERROR]: Unknown CalendarView in Julian converter (to_date)")
        }

        self.fill_time(UNIX_TIME_START_AFTER_DAY + ALIGN_JULIAN as u128, tz_in_unixtime);
        self.fill_date(CalendarView::Julian);
    }

    fn to_presentation(&mut self, tz_in_unixtime: bool) {
        let last_year: u128 = if self.year > 0 { self.year - 1 } else { 0 };
        
        match self.view {
            CalendarView::Julian => {
                self.era_days = era_days_from_date(self.view.clone(), self.year, self.month, self.day);
            },
            CalendarView::Gregorian => {
                // Конвертируем из Григорианского в Юлианский (в Григорианском меньше високосных дней, но представление больше)
                // Добавляем к Григорианским дням, отсутствующие в Григорианском календаре високосные
                // дни Юлианского календаря
                self.era_days = 365_u128 * last_year;
                self.era_days += ((last_year * 2425_u128) as f64 / 10000.0_f64 + (last_year * 75_u128) as f64 / 10000.0_f64) as u128;
                self.era_days += days_from_date(CalendarView::Julian, self.year, self.month, self.day);
            },
            CalendarView::Solar => {
                // Конвертируем из Солнечного в Юлианский (в Солнечном меньше високосных дней, но представление больше)
                // Добавляем к Солнечным дням, отсутствующие в Солнечном календаре високосные дни Юлианского календаря
                self.era_days = 365_u128 * last_year;
                self.era_days += ((last_year * 24219_u128) as f64 / 100000.0_f64 + ((last_year * 781_u128) as f64 / 100000.0_f64)) as u128;
                self.era_days += days_from_date(CalendarView::Julian, self.year, self.month, self.day);
            },
            _ => panic!("[ERROR]: Unknown CalendarView in Julian converter (to_presentation)")
        }

        self.fill_time(UNIX_TIME_START_AFTER_DAY + ALIGN_JULIAN as u128, tz_in_unixtime);
        self.fill_date(CalendarView::Julian);
    }
}