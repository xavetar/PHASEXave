/* MIT License
 * 
 * Copyright (c) 2023 Stanislav Mikhailov (xavetar)
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Алгоритм для вычисления дня недели (Julian calendar):

// Таблица:

// January   -  5  = (5) mod 7
// February  -  1  = (5 + 31) mod 7
// March     - 1~2 = (5 + 31 + [28, 29]) mod 7
// April     - 4-5 = (5 + 31 + [28, 29] + 31) mod 7
// May       - 6~0 = (5 + 31 + [28, 29] + 31 + 30) mod 7
// June      - 2~3 = (5 + 31 + [28, 29] + 31 + 30 + 31) mod 7
// July      - 4~5 = (5 + 31 + [28, 29] + 31 + 30 + 31 + 30) mod 7
// August    - 0~1 = (5 + 31 + [28, 29] + 31 + 30 + 31 + 30 + 31) mod 7
// September - 3~4 = (5 + 31 + [28, 29] + 31 + 30 + 31 + 30 + 31 + 31) mod 7
// October   - 5~6 = (5 + 31 + [28, 29] + 31 + 30 + 31 + 30 + 31 + 31 + 30) mod 7
// November  - 1~2 = (5 + 31 + [28, 29] + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31) mod 7
// December  - 3~4 = (5 + 31 + [28, 29] + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30) mod 7
//                        31   [28, 29]   31   30   31   30   31   31   30   31   30

// Описание:

// Вычисление дня недели согласно Юлианскому календарю, без учёта коррекций по Григорианскому календарю.
// На многих ресурсах даны неправильные алгоритмы или посредственные, учитывающие коррекцию начиная с 1582 года
// Вводя новую концепцию в этот календарь, безусловно - она верная и блаблабла, но это не отменяет противоречия
// и подмены понятий. ИМЕННО в Григорианском календаре были пропущенны 10 дней (04.10.1582=>15.10.1582), НЕ в Юлианском.
// Это и многое другое, критически противоречит концепции календаря, его эталонной модели, где каждый 4-й год високосный.

// Ресурсы и калькуляторы, рассуждения:

// 1. http://www.stevegs.com/utils/jd_calc/
// 2. https://www.timeanddate.com/
// 3. https://www.thecalculatorsite.com/

// 1. stevegs - учитывает коррекции начиная с (04.10.1582=>15.10.1582) и возможно некоторые другие, что нарушает
// стандартную концепцию Юлианского календаря и противоречит его идее.
// Ведь фактически подразумевается автором, что мы вводим дату на Григорианском, а выдаёт он нам её в Юлианском,
// тогда почему автор это калькулятора решил учитывать Григорианский календарь до 1582?! Где логика?!
// Фактически должно БЫТЬ, а не подразумеваться, что мы вводим дату из Юлианского календаря и получаем день недели
// в Юлианском календаре, без мнимых конвертации и нарушении его базовой логики.
// Алгоритмы описанные и адаптированные в этом разделе, идеально вычисляют дату до 04.10.1582, согласно stevegs,
// после возникают проблемы, ибо автор предпочёл учитывать коррекции.

const MONTHS_IN_YEAR: u8 = 12;
const REPEAT_WEAK_DAY_CYCLE: u8 = 7;

const SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN: u8 = 6 - 1;

const BASE_MONTH_DAYS: [u8; MONTHS_IN_YEAR as usize] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LEAP_MONTH_DAYS: [u8; MONTHS_IN_YEAR as usize] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

// Base year months mod by 7 (365 days) - [5, 1, 1, 4, 6, 2, 4, 0, 3, 5, 1, 3]
const BASE: [u8; MONTHS_IN_YEAR as usize] = [
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16 + BASE_MONTH_DAYS[7] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16 + BASE_MONTH_DAYS[7] as u16 + BASE_MONTH_DAYS[8] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16 + BASE_MONTH_DAYS[7] as u16 + BASE_MONTH_DAYS[8] as u16 + BASE_MONTH_DAYS[9] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16 + BASE_MONTH_DAYS[7] as u16 + BASE_MONTH_DAYS[8] as u16 + BASE_MONTH_DAYS[9] as u16 + BASE_MONTH_DAYS[10] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8
];

// Leap year months mod by 7 (366 days) - [5, 1, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4]
const LEAP: [u8; MONTHS_IN_YEAR as usize] = [
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + LEAP_MONTH_DAYS[0] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16 + LEAP_MONTH_DAYS[5] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16 + LEAP_MONTH_DAYS[5] as u16 + LEAP_MONTH_DAYS[6] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16 + LEAP_MONTH_DAYS[5] as u16 + LEAP_MONTH_DAYS[6] as u16 + LEAP_MONTH_DAYS[7] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16 + LEAP_MONTH_DAYS[5] as u16 + LEAP_MONTH_DAYS[6] as u16 + LEAP_MONTH_DAYS[7] as u16 + LEAP_MONTH_DAYS[8] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16 + LEAP_MONTH_DAYS[5] as u16 + LEAP_MONTH_DAYS[6] as u16 + LEAP_MONTH_DAYS[7] as u16 + LEAP_MONTH_DAYS[8] as u16 + LEAP_MONTH_DAYS[9] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_JULIAN as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16 + LEAP_MONTH_DAYS[5] as u16 + LEAP_MONTH_DAYS[6] as u16 + LEAP_MONTH_DAYS[7] as u16 + LEAP_MONTH_DAYS[8] as u16 + LEAP_MONTH_DAYS[9] as u16 + LEAP_MONTH_DAYS[10] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8
];

const fn is_leap_year_julian(year: u128) -> bool {
    return year % 4 == 0;
}
 
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
    let last_year: u128 = year - 1; // Для вычисления смещение в Январе (это можно сделать только с предыдущего года)
                                    // To calculate the offset in January (this can only be done from the previous year)

    let sum_leap_years_julian: u128 = last_year / 4;
    let shift_week_day_in_jan: u8 = ((last_year + sum_leap_years_julian) % REPEAT_WEAK_DAY_CYCLE as u128) as u8;

    let shift_week_day_in_month_and_day: u8;

    if !is_leap_year_julian(year) {
        shift_week_day_in_month_and_day = (BASE[(month - 1) as usize] + day) % REPEAT_WEAK_DAY_CYCLE;
        
        print_debug(
            "BASE",
            shift_week_day_in_jan, 
            shift_week_day_in_month_and_day,
            (shift_week_day_in_jan + shift_week_day_in_month_and_day) % REPEAT_WEAK_DAY_CYCLE
        );

        return week_day((shift_week_day_in_jan + shift_week_day_in_month_and_day) % REPEAT_WEAK_DAY_CYCLE);
    } else if is_leap_year_julian(year) {
        shift_week_day_in_month_and_day = (LEAP[(month - 1) as usize] + day) % REPEAT_WEAK_DAY_CYCLE;

        print_debug(
            "LEAP",
            shift_week_day_in_jan, 
            shift_week_day_in_month_and_day,
            (shift_week_day_in_jan + shift_week_day_in_month_and_day) % REPEAT_WEAK_DAY_CYCLE
        );

        return week_day((shift_week_day_in_jan + shift_week_day_in_month_and_day) % REPEAT_WEAK_DAY_CYCLE);
    }

    panic!("Unkonwn error");
}

fn main() {
    println!("{}", day_of_week(2024, 2, 29));
}

// Сокращенный вариант для понимания:

const REPEAT_WEAK_DAY_CYCLE: u8 = 7;

const BASE: [u8; 12] = [5, 1, 1, 4, 6, 2, 4, 0, 3, 5, 1, 3];
const LEAP: [u8; 12] = [5, 1, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

const fn is_leap_year_julian(year: u128) -> bool {
    return year % 4 == 0;
}
 
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
 
fn day_of_week(year: u128, month: u8, day: u8) -> &'static str {
    let last_year: u128 = year - 1;

    let sum_leap_years_julian: u128 = last_year / 4;
    let shift_week_day_in_jan: u8 = ((last_year + sum_leap_years_julian) % REPEAT_WEAK_DAY_CYCLE as u16 as u128) as u8;

    let shift_week_day_in_month_and_day: u8;

    if !is_leap_year_julian(year) {
        shift_week_day_in_month_and_day = (BASE[(month - 1) as usize] + day) % REPEAT_WEAK_DAY_CYCLE as u16;
        return week_day((shift_week_day_in_jan + shift_week_day_in_month_and_day) % REPEAT_WEAK_DAY_CYCLE as u16);
    } else if is_leap_year_julian(year) {
        shift_week_day_in_month_and_day = (LEAP[(month - 1) as usize] + day) % REPEAT_WEAK_DAY_CYCLE as u16;
        return week_day((shift_week_day_in_jan + shift_week_day_in_month_and_day) % REPEAT_WEAK_DAY_CYCLE as u16);
    }

    panic!("Unkonwn error");
}
 
fn main() {
    println!("{}", day_of_week(2024, 3, 29));
}

// Оптимизированная версия:

const REPEAT_WEAK_DAY_CYCLE: u8 = 7;

const BASE: [u8; 12] = [5, 1, 1, 4, 6, 2, 4, 0, 3, 5, 1, 3];
const LEAP: [u8; 12] = [5, 1, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

const fn is_leap_year_julian(year: u128) -> bool {
    return year % 4 == 0;
}

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

fn day_of_week(year: u128, month: u8, day: u8) -> &'static str {
    let last_year: u128 = year - 1;

    if !is_leap_year_julian(year) {
        return week_day(((last_year + (year / 4) + BASE[(month - 1) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u16 as u128) as u8);
    } else if is_leap_year_julian(year) {
        return week_day(((last_year + (year / 4) + LEAP[(month - 1) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u16 as u128) as u8);
    }

    panic!("Unkonwn error");
}

fn main() {
    println!("{}", day_of_week(2024, 3, 29));
}