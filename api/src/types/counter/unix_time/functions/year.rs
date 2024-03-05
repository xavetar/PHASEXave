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

use crate::types::{
    planets::earth::calendar::{
        view::{CalendarView},
        constants::year::{BASE_DAYS_YEAR, LEAP_DAYS_YEAR},
        functions::{is_leap_year}
    }
};

/*
    Название было изменено с year_from_era_days, фактически мы получаем из результата работы этой функции
    количество/сумму лет с представления даты в какой-то календарной системе, но в самом представлении даты,
    могут находится дни другой эры, именно поэтому такое название было бы не совсем корректным.

    В Юлианском календаре это дни 01.01.1 BCE и 02.01.1 BCE, таким образом по факту функция возвращает
    год, в представления даты в какой-то календарной системе. И этот результат должен быть скорректирован
    в функциях выше, для приведения к формату CE (Current Era) или BCE (Before Current Era)*.

    Для Григорианского и Солнечного данные логические утверждения так-же верны, ведь правильное количество/сумма
    лет для этой эры, является подмножеством презинтативной даты, зависящей от количества дней в году, не смотря
    на то, что для этих календарей это ничего не меняет, для них можно считать что название функции так и осталось
    year_from_era_days, т.к в них отсутствуют BCE (Before Current Era) дни/года.

    * - данная функциональность ПОКА не реализована.
*/

pub fn year_from_presentation_days(view: CalendarView, presentation_days: u128) -> (u128, u128) {
    let mut year: u128 = 1_u128;
    let mut days: u128 = presentation_days;

    loop {
        let leap_year: bool = is_leap_year(view, year);

        if leap_year && days > LEAP_DAYS_YEAR as u128 {
            days -= LEAP_DAYS_YEAR as u128;
            year += 1_u128;
        } else if !leap_year && days > BASE_DAYS_YEAR as u128 {
            days -= BASE_DAYS_YEAR as u128;
            year += 1_u128;
        } else {
            break;
        }
    };

    return (year, days);
}