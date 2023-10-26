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
        constants::{
            year::{
                SOLAR_YEAR_LEAP_LENGTH_INT,
                SOLAR_YEAR_LEAP_LENGTH_F64
            }
        }
    }
};

/*
    - В Юлианском календаре:

    Каждый 4-й год високосный, поэтому мы можем однозначно детерминировать високосный год,
    взяв остаток от деления на 4.

    Этот вид календаря давно потерял свою точность с точки зрения соотнесения представления календаря
    и его даты с реальным временем года. Для остальных целей, если не важно представление даты и равенство
    сезонов между годами, он подходит идеально. Всё равно из-за теории больших чисел он рано или поздно
    вернётся к точке отсчёта сезонов и представления даты.

    - В Григорианском календаре:

    Каждый 4-й год високосный, за исключением года, который делится на 100 (без остатка),
    при этом если год делится без остатка на 400 (кратен 400), год будет високосным (игнорируется правило 100).
    Григорианский календарь, уже сейчас опережает тропический год на 1 день. Теория больших чисел говорит о том,
    что бесполезно в динамической системе, использовать статические средства определения каких либо значений.
    Григорианский календарь, либо закончит своё существование вставками, либо его заменит другой более точный
    календарь.

    - В Солнечном календаре:

    Он же Тропически, в нём сложно или невозможно однозначно определить високосный год из-за большой
    високосной части - 0.24219(...). И даже если это возможно, то расхождения представления календаря
    и времён года - избежать будет крайне сложно. Поэтому единственный и самый гениальный вариант,
    однозначного определения, идти от обратного (для этого нужен был инсайт).

    К примеру, за первые 4-ре года в Солнечном/Тропическом календаре проходит:

    4 * 0.24219 = 0.96876 (високосных дней)

    Из-за чего 4-й год не может быть високосным - априори, но вот следующий за ним - вполне.

    Как работает формула и почему она работает? Формула работает так:

    1) Берёт текущий год, к примеру 5-й (year)
    2) Берёт предыдущий год, 4-й
    3) Вычисляет вещественное високосное значение для предыдущего года, 0.96876
    4) Добавляет вещественную високосную часть (0.24219) от следующего года (5-й) к 0.96876,
       что равно 1.21095 (на конец 5-го года).
    5) Сравнивает целые части:
    5.1) Когда целая високосная часть следующего года (1), больше, чем за предыдущий год (0): 1 > 0, год високосный.

    Таким образом точность календарной системы, теперь зависит только от констатны високосной части года, для
    Солнечного календаря она определена как 0.24219. Что хорошо, так как это избавляет от ситуаций, при которых
    делают вставки и одновременно кратно повышает точность календарной системы.
*/

pub fn is_leap_year(view: CalendarView, year: u128) -> bool {
    match view {
        CalendarView::Julian => return year % 4_u128 == 0_u128,
        CalendarView::Gregorian => return year % 4_u128 == 0_u128 && (year % 100_u128 != 0_u128 || year % 400_u128 == 0_u128),
        CalendarView::Solar => {
            if year != 0 {
                let leap_years: f64 = ((year - 1_u128) * SOLAR_YEAR_LEAP_LENGTH_INT) as f64 / 100000.0_f64;

                if (leap_years + SOLAR_YEAR_LEAP_LENGTH_F64) as u128 > (leap_years) as u128 {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false
            }
        },
        _ => panic!("[ERROR]: Unknown CalendarView (is_leap_year)!")
    }
}

pub fn sum_leap_years(view: CalendarView, year: u128) -> u128 {
    match view {
        CalendarView::Julian => return year / 4_u128,
        CalendarView::Gregorian => return year / 4_u128 - year / 100_u128 + year / 400_u128,
        CalendarView::Solar => return (year * SOLAR_YEAR_LEAP_LENGTH_INT) / 100000_u128,
        _ => panic!("[ERROR]: Unknown CalendarView (sum_leap_years)!")
    }
}

pub const fn excess_leap_years(year: u128) -> u128 {
    return year / 100_u128 - year / 400_u128;
}
