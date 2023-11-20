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

use super::{zone_recalc};

use crate::types::{
    data::{
        date::{Date},
    },
    planets::earth::calendar::{
        view::{CalendarView},
        constants::{
            seconds::{SECONDS_IN_DAY},
            days::{ALIGN_JULIAN_TO_GREGORIAN_DAYS}
        },
        functions::{era_days_from_date}
    },
    counter::unix_time::{
        constants::{
            days::{UNIX_DAYS_BEFORE_EPOCH_JULIAN}
        },
        functions::{
            year_from_days, month_from_days
        }
    }
};

/*
    Signed Timezone:

    1. Если временная зона отрицательна и количество секунд в последнем дне (по UTC - unix time) больше или равно,
    чем во временной зоне, то отнимается количество секунд временной зоны, и прибавляется количество секунд за текущий день,
    при этом количество дней не изменяется! era_days: [0, 0].

    2. Если временная зона содержит больше (отрицательных по отношению к UTC) секунд и количество секунд в последнем дне (day_seconds)
    меньше, чем во временной зоне (т.е это всегда от [-1, -N] дней к секундам по UTC - unix time), то вычисляется разница, между
    временной зоной и секундами в текущем дне - разница потому-что эти секунды всегда идут вперёд, тем самым откусывая часть времени
    от временной зоны вперёд, в результате разницы, получается значение/сумма секунд, на которое текущий день по UTC отходит назад по
    unix time. Минимальное значение этой разницы 1 секунда (23:59:59) или 1 день назад. Деление с округлением вверх - это трюк и
    попытка получить, абсолютную сумму дней отходящих назад. Причём это выражение всегда находится в пределе от [-1, -N] дней, по
    отношению к дням эры/секундам от/по UTC, вне зависимости от временной зоны! era_days: [-1, -N].
    После этого отнимается количество секунд временной зоны, и прибавляется количество секунд за текущий день.

    Unsigned Timezone:

    1. Если временная зона положительна (по отношению к UTC) и общая сумма секунд временной зоны (tz_sec) и последнем дне (day_seconds),
    строго меньше/не превышает, максимальное количество секунд, требуемое для смены дня (SECONDS_IN_DAY) - день остаётся прежним,
    при этом количество дней не изменяется! era_days: [0, 0].

    2. Если временная зона положительна (по отношению к UTC) и общая сумма секунд временной зоны (tz_sec) и последнем дне (day_seconds),
    больше/превышает или равна, минимальному значению требуемому для смены дня (SECONDS_IN_DAY), выполняется Евклидово деление
    целочисленное/с округлением вниз - суммы секунд временной зоны (tz_sec) и последнем дне (day_seconds) на SECONDS_IN_DAY, с целью
    узнать сумму дней превышения (т.е это всегда от [+1, +N] дней к секундам по UTC - unix time) и увеличивается количество дней эпохи
    на это значение. Причём это выражение всегда находится в пределе от [+1, +N] дней, по отношению к дням эры/секундам от/по UTC, вне
    зависимости от временной зоны! era_days: [+1, +N].
    После этого прибавляется количество секунд временной зоны, и количество секунд за текущий день.
*/

pub trait Julian {
    fn to_julian(&mut self, tz_in_unixtime: bool);
}

impl Julian for Date {
    fn to_julian(&mut self, tz_in_unixtime: bool) {
        let mut days: u128;

        match self.view {
            CalendarView::Julian => {
                // Вычисление дней эры
                self.era_days = era_days_from_date(self.view.clone(), self.year, self.month, self.day);
            },
            CalendarView::Gregorian => {
                // Вычисление дней эры и добавление отсутствующих дней в Григорианском календаре (+1 находится в самой дате)
                self.era_days = era_days_from_date(self.view.clone(), self.year, self.month, self.day) + ALIGN_JULIAN_TO_GREGORIAN_DAYS as u128;
            },
            _ => panic!("[ERROR]: Unknown CalendarView (to_julian)")
        }

        if self.era_days > UNIX_DAYS_BEFORE_EPOCH_JULIAN {
            let day_seconds: u128 = self.unix_time % SECONDS_IN_DAY;

            self.unix_time = (self.era_days - (UNIX_DAYS_BEFORE_EPOCH_JULIAN + 1)) * SECONDS_IN_DAY;

            // Используется в случае когда временная зона не находится в unix time, позволяет указать время внутри дня,
            // с учётом секунд внутри дня ± часовой пояс.
            if !tz_in_unixtime {
                zone_recalc(self.timezone, &mut self.unix_time, day_seconds, &mut self.era_days);
            } else {
                self.unix_time += day_seconds;
            }
        } else {
            self.unix_time = 0;
        }

        // Внутри функции происходит неявное смещение, из-за чего использование функции excess_leap_years является излишним
        // и может привести к неточностям. Этот метод основан на подсчёте дней, он действителен для любой даты и универсален.
        (self.year, days) = year_from_days(CalendarView::Julian, self.era_days);

        self.month = month_from_days(CalendarView::Julian, self.year, &mut days).index();

        self.day = days as u8;

        self.view = CalendarView::Julian;
    }
}