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

// Алгоритм для вычисления дня недели (Julian calendar):

// Преварительные расчёты:

// January   -  5  = (5) mod 7
// February  -  1  = (5 + 31) mod 7
// March     -  0  = (5 + 31 + (28 - 1)) mod 7
// April     -  3  = (5 + 31 + (28 - 1) + 31) mod 7
// May       -  5  = (5 + 31 + (28 - 1) + 31 + 30) mod 7
// June      -  1  = (5 + 31 + (28 - 1) + 31 + 30 + 31) mod 7
// July      -  3  = (5 + 31 + (28 - 1) + 31 + 30 + 31 + 30) mod 7
// August    -  6  = (5 + 31 + (28 - 1) + 31 + 30 + 31 + 30 + 31) mod 7
// September -  2  = (5 + 31 + (28 - 1) + 31 + 30 + 31 + 30 + 31 + 31) mod 7
// October   -  4  = (5 + 31 + (28 - 1) + 31 + 30 + 31 + 30 + 31 + 31 + 30) mod 7
// November  -  0  = (5 + 31 + (28 - 1) + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31) mod 7
// December  -  2  = (5 + 31 + (28 - 1) + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30) mod 7
//                        31    28 - 1    31   30   31   30   31   31   30   31   30

// Описание:

// Смещение по годам (shift_week_day_by_years): Этот компонент учитывает влияние годов и високосных лет на день недели.
// Он вычисляется как сумма текущего года и коррекции, последняя зависит от количества високосных лет в заданном диапазоне.
// Формула (year + (year / 4)) % REPEAT_WEAK_DAY_CYCLE as u16 позволяет учесть это смещение.

// Смещение по дням месяца (shift_week_day_by_month): Этот компонент учитывает количество дней в месяце и день месяца.
// Он вычисляется как сумма смещения для начала месяца и номера дня в месяце.
// Формула (base[(month - 1) as usize] + day as u16) % REPEAT_WEAK_DAY_CYCLE as u16 учитывает это смещение.

// Итоговое смещение (shift): Этот компонент объединяет смещение по годам и смещение по дням месяца.
// Он вычисляется как сумма shift_week_day_by_years и shift_week_day_by_month, затем берется остаток от деления на 7.
// Это итоговое смещение представляет собой день недели для заданной даты, где 0 - воскресенье, 1 - понедельник и так далее.

// Для оптимизации кода данная формула, оптимизируется до формата:
// ((year + (year / 4)) + (days_by_month_mod[(month - 1) as usize] + day as u16)) % REPEAT_WEAK_DAY_CYCLE as u16

// Запоздалое дополнение (после написания своего алгоритма):

// По факту получается, оптимизированный алгоритм Сакамото, в случае, если месяц > 3, вычисляет смещение для Января в следующем году
// (~2023.05 => 2024.05) и суммирует с новым смещением для месяца, для этого мы отнимали (-1), для месяца после Марта, чтобы было опережающее вычисление.
// Если месяц < 3, алгоритм вычисляет смещение для текущего года с пердыдущего (как и в моём алгоритме, где я выравнял эти значения) 
// Фактически переменная: "shift_week_day_in_jan", должна называться: "shift_week_day_in_next_jan", когда месяц >= 3.
// Для понимания этого алгоритма, пришлось потратить несколько дней, т.к некачественные ответы по ссылкам выше и на других ресурсах,
// хорошего не делают, а только разжижают мозги, лучше никакой ответ, чем неправильный, думайте сами!

// * Ссылки (Gregorian Calendar):

// https://www.quora.com/How-does-Tomohiko-Sakamotos-Algorithm-work
// https://iq.opengenus.org/tomohiko-sakamoto-algorithm/
// https://leetcode.com/problems/day-of-the-week/solutions/381894/JavaC++Python3-Sakamoto-Algorithm/comments/343457/
// https://www.tutorialspoint.com/tomohiko-sakamoto-rsquo-s-algorithm-finding-the-day-of-the-week

const MONTHS_IN_YEAR: u8 = 12;
const REPEAT_WEAK_DAY_CYCLE: u8 = 7;

const SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN: u8 = 6 - 1;

const BASE_MONTH_DAYS: [u8; MONTHS_IN_YEAR as usize] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

// Base year months mod by 7 and (-1) - [5, 1, 0, 3, 5, 1, 3, 6, 2, 4, 0, 2]
const JULIAN_BASE: [u8; MONTHS_IN_YEAR as usize] = [
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + ((BASE_MONTH_DAYS[1] as u16) - 1)) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + ((BASE_MONTH_DAYS[1] as u16) - 1) + BASE_MONTH_DAYS[2] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + ((BASE_MONTH_DAYS[1] as u16) - 1) + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + ((BASE_MONTH_DAYS[1] as u16) - 1) + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + ((BASE_MONTH_DAYS[1] as u16) - 1) + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + ((BASE_MONTH_DAYS[1] as u16) - 1) + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + ((BASE_MONTH_DAYS[1] as u16) - 1) + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16 + BASE_MONTH_DAYS[7] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + ((BASE_MONTH_DAYS[1] as u16) - 1) + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16 + BASE_MONTH_DAYS[7] as u16 + BASE_MONTH_DAYS[8] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + ((BASE_MONTH_DAYS[1] as u16) - 1) + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16 + BASE_MONTH_DAYS[7] as u16 + BASE_MONTH_DAYS[8] as u16 + BASE_MONTH_DAYS[9] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + ((BASE_MONTH_DAYS[1] as u16) - 1) + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16 + BASE_MONTH_DAYS[7] as u16 + BASE_MONTH_DAYS[8] as u16 + BASE_MONTH_DAYS[9] as u16 + BASE_MONTH_DAYS[10] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8
];

const fn week_day(shift: u8) -> &'static str {
    return match shift {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => panic!("Invalid day"),
    };
}

fn print_debug(f: &'static str, shift_week_day_in_jan: u8, shift_week_day_in_month_and_day: u8, week_day_in_date: u8) {
    println!("First week day in year ({}): {}", f, week_day((shift_week_day_in_jan + 1) % REPEAT_WEAK_DAY_CYCLE));
    println!("Shift week day by year ({}): {}", f, week_day(shift_week_day_in_jan));

    println!("Week day in month ({}): {}", f, week_day(week_day_in_date));

    println!("Shift by years ({}): {}", f, shift_week_day_in_jan);
    println!("Shift by days ({}): {}", f, shift_week_day_in_month_and_day);
}

fn day_of_week(year: u128, month: u8, day: u8) -> &'static str {
    let mut local_year: u128 = year;

    if month < 3 { local_year -= 1; }

    let sum_leap_years_julian: u128 = local_year / 4;
    let shift_week_day_in_jan: u8 = ((local_year + sum_leap_years_julian) % REPEAT_WEAK_DAY_CYCLE as u128) as u8; // Выдаёт результаты, в которых можно запутаться, из-за оптимизаций

    let shift_week_day_in_month_and_day: u8 = (JULIAN_BASE[(month - 1) as usize] + day) % REPEAT_WEAK_DAY_CYCLE;

    print_debug(
        "BASE",
        shift_week_day_in_jan, 
        shift_week_day_in_month_and_day,
        (shift_week_day_in_jan + shift_week_day_in_month_and_day) % REPEAT_WEAK_DAY_CYCLE
    );
    
    return week_day((shift_week_day_in_jan + shift_week_day_in_month_and_day) % REPEAT_WEAK_DAY_CYCLE);
}

fn main() {
    println!("{}", day_of_week(2024, 2, 29));
}

// Оптимизированный вариант:

const REPEAT_WEAK_DAY_CYCLE: u8 = 7;

fn day_of_week(mut year: u16, month: u8, day: u8) -> &'static str {
    let base: [u16; 12] = [5, 1, 0, 3, 5, 1, 3, 6, 2, 4, 0, 2];

    if month < 3 {
        year -= 1;
    }
    let v: u16 = (year + (year / 4) + base[(month - 1) as usize] + day as u16) % REPEAT_WEAK_DAY_CYCLE as u16;

    return match v {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => panic!("Invalid day"),
    };
}
