/* MIT License
 * 
 * Copyright (c) 2024 Stanislav Mikhailov (xavetar)
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

// Алгоритм для вычисления дня недели (Solar calendar):

// Таблица:

// January   -  0  = (0) mod 7
// February  -  3  = (0 + 31) mod 7
// March     - 3~4 = (0 + 31 + [28, 29]) mod 7
// April     - 6-0 = (0 + 31 + [28, 29] + 31) mod 7
// May       - 1~2 = (0 + 31 + [28, 29] + 31 + 30) mod 7
// June      - 4~5 = (0 + 31 + [28, 29] + 31 + 30 + 31) mod 7
// July      - 6~0 = (0 + 31 + [28, 29] + 31 + 30 + 31 + 30) mod 7
// August    - 2~3 = (0 + 31 + [28, 29] + 31 + 30 + 31 + 30 + 31) mod 7
// September - 5~6 = (0 + 31 + [28, 29] + 31 + 30 + 31 + 30 + 31 + 31) mod 7
// October   - 0~1 = (0 + 31 + [28, 29] + 31 + 30 + 31 + 30 + 31 + 31 + 30) mod 7
// November  - 3~4 = (0 + 31 + [28, 29] + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31) mod 7
// December  - 5~6 = (0 + 31 + [28, 29] + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30) mod 7
//                        31   [28, 29]   31   30   31   30   31   31   30   31   30

const MONTHS_IN_YEAR: u8 = 12;
const REPEAT_WEAK_DAY_CYCLE: u8 = 7;

const SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR: u8 = 1 - 1;

const BASE_MONTH_DAYS: [u8; MONTHS_IN_YEAR as usize] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LEAP_MONTH_DAYS: [u8; MONTHS_IN_YEAR as usize] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

// Base year months mod by 7 (365 days) - [0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5]
const BASE: [u8; MONTHS_IN_YEAR as usize] = [
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + BASE_MONTH_DAYS[0] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16 + BASE_MONTH_DAYS[7] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16 + BASE_MONTH_DAYS[7] as u16 + BASE_MONTH_DAYS[8] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16 + BASE_MONTH_DAYS[7] as u16 + BASE_MONTH_DAYS[8] as u16 + BASE_MONTH_DAYS[9] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + BASE_MONTH_DAYS[0] as u16 + BASE_MONTH_DAYS[1] as u16 + BASE_MONTH_DAYS[2] as u16 + BASE_MONTH_DAYS[3] as u16 + BASE_MONTH_DAYS[4] as u16 + BASE_MONTH_DAYS[5] as u16 + BASE_MONTH_DAYS[6] as u16 + BASE_MONTH_DAYS[7] as u16 + BASE_MONTH_DAYS[8] as u16 + BASE_MONTH_DAYS[9] as u16 + BASE_MONTH_DAYS[10] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8
];

// Leap year months mod by 7 (366 days) - [0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6]
const LEAP: [u8; MONTHS_IN_YEAR as usize] = [
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + LEAP_MONTH_DAYS[0] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16 + LEAP_MONTH_DAYS[5] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16 + LEAP_MONTH_DAYS[5] as u16 + LEAP_MONTH_DAYS[6] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16 + LEAP_MONTH_DAYS[5] as u16 + LEAP_MONTH_DAYS[6] as u16 + LEAP_MONTH_DAYS[7] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16 + LEAP_MONTH_DAYS[5] as u16 + LEAP_MONTH_DAYS[6] as u16 + LEAP_MONTH_DAYS[7] as u16 + LEAP_MONTH_DAYS[8] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16 + LEAP_MONTH_DAYS[5] as u16 + LEAP_MONTH_DAYS[6] as u16 + LEAP_MONTH_DAYS[7] as u16 + LEAP_MONTH_DAYS[8] as u16 + LEAP_MONTH_DAYS[9] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8,
    ((SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u16 + LEAP_MONTH_DAYS[0] as u16 + LEAP_MONTH_DAYS[1] as u16 + LEAP_MONTH_DAYS[2] as u16 + LEAP_MONTH_DAYS[3] as u16 + LEAP_MONTH_DAYS[4] as u16 + LEAP_MONTH_DAYS[5] as u16 + LEAP_MONTH_DAYS[6] as u16 + LEAP_MONTH_DAYS[7] as u16 + LEAP_MONTH_DAYS[8] as u16 + LEAP_MONTH_DAYS[9] as u16 + LEAP_MONTH_DAYS[10] as u16) % REPEAT_WEAK_DAY_CYCLE as u16) as u8
];

fn is_leap_year_solar(year: u128) -> bool {
    if year != 0 {
        let last_year: u128 = year - 1;

        let leap_years: f64 = (last_year as f64) / 4.0_f64 - (last_year as f64) / 100.0_f64 + (last_year as f64) / 400.0_f64 - (last_year as f64) / 4000.0_f64 - (last_year as f64) / 20000.0_f64 - (last_year as f64) / 100000.0_f64;

        if (leap_years + 0.24219_f64) as u128 > (leap_years) as u128 {
            return true;
        } else {
            return false;
        }
    } else {
        return false
    }
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

    let sum_leap_years_solar: u128 = ((last_year as f64) / 4.0_f64 - (last_year as f64) / 100.0_f64 + (last_year as f64) / 400.0_f64 - (last_year as f64) / 4000.0_f64 - (last_year as f64) / 20000.0_f64 - (last_year as f64) / 100000.0_f64) as u128;
    let shift_week_day_in_jan: u8 = ((last_year + sum_leap_years_solar) % REPEAT_WEAK_DAY_CYCLE as u128) as u8;

    let shift_week_day_in_month_and_day: u8;

    println!("{}", is_leap_year_solar(year));

    if !is_leap_year_solar(year) {
        shift_week_day_in_month_and_day = (BASE[(month - 1) as usize] + day) % REPEAT_WEAK_DAY_CYCLE;

        print_debug(
            "BASE",
            shift_week_day_in_jan,
            shift_week_day_in_month_and_day,
            (shift_week_day_in_jan + shift_week_day_in_month_and_day) % REPEAT_WEAK_DAY_CYCLE
        );

        return week_day((shift_week_day_in_jan + shift_week_day_in_month_and_day) % REPEAT_WEAK_DAY_CYCLE);
    } else {
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
    println!("{}", day_of_week(322600000, 12, 31));
}

// Сокращенный вариант для понимания:

const REPEAT_WEAK_DAY_CYCLE: u8 = 7;

const BASE: [u8; 12] = [0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5];
const LEAP: [u8; 12] = [0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6];

fn is_leap_year_solar(year: u128) -> bool {
    if year != 0 {
        let last_year: u128 = year - 1;

        let leap_years: f64 = (last_year as f64) / 4.0_f64 - (last_year as f64) / 100.0_f64 + (last_year as f64) / 400.0_f64 - (last_year as f64) / 4000.0_f64 - (last_year as f64) / 20000.0_f64 - (last_year as f64) / 100000.0_f64;

        if (leap_years + 0.24219_f64) as u128 > (leap_years) as u128 {
            return true;
        } else {
            return false;
        }
    } else {
        return false
    }
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

    let sum_leap_years_solar: u128 = ((last_year as f64) / 4.0_f64 - (last_year as f64) / 100.0_f64 + (last_year as f64) / 400.0_f64 - (last_year as f64) / 4000.0_f64 - (last_year as f64) / 20000.0_f64 - (last_year as f64) / 100000.0_f64) as u128;
    let shift_week_day_in_jan: u8 = ((last_year + sum_leap_years_solar) % REPEAT_WEAK_DAY_CYCLE as u128) as u8;

    let shift_week_day_in_month_and_day: u8;

    if !is_leap_year_solar(year) {
        shift_week_day_in_month_and_day = (BASE[(month - 1) as usize] + day) % REPEAT_WEAK_DAY_CYCLE;
        return week_day((shift_week_day_in_jan + shift_week_day_in_month_and_day) % REPEAT_WEAK_DAY_CYCLE);
    } else {
        shift_week_day_in_month_and_day = (LEAP[(month - 1) as usize] + day) % REPEAT_WEAK_DAY_CYCLE;
        return week_day((shift_week_day_in_jan + shift_week_day_in_month_and_day) % REPEAT_WEAK_DAY_CYCLE);
    }

    panic!("Unkonwn error");
}

fn main() {
    println!("{}", day_of_week(322600000, 12, 31));
}

// Оптимизированная версия:

const REPEAT_WEAK_DAY_CYCLE: u8 = 7;

const BASE: [u8; 12] = [0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5];
const LEAP: [u8; 12] = [0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6];

fn is_leap_year_solar(year: u128) -> bool {
    if year != 0 {
        let last_year: u128 = year - 1;

        let leap_years: f64 = (last_year as f64) / 4.0_f64 - (last_year as f64) / 100.0_f64 + (last_year as f64) / 400.0_f64 - (last_year as f64) / 4000.0_f64 - (last_year as f64) / 20000.0_f64 - (last_year as f64) / 100000.0_f64;

        if (leap_years + 0.24219_f64) as u128 > (leap_years) as u128 {
            return true;
        } else {
            return false;
        }
    } else {
        return false
    }
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

    if !is_leap_year_solar(year) {
        return week_day(((last_year + ((last_year as f64) / 4.0_f64 - (last_year as f64) / 100.0_f64 + (last_year as f64) / 400.0_f64 - (last_year as f64) / 4000.0_f64 - (last_year as f64) / 20000.0_f64 - (last_year as f64) / 100000.0_f64) as u128 + BASE[(month - 1) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u128) as u8);
    } else {
        return week_day(((last_year + ((last_year as f64) / 4.0_f64 - (last_year as f64) / 100.0_f64 + (last_year as f64) / 400.0_f64 - (last_year as f64) / 4000.0_f64 - (last_year as f64) / 20000.0_f64 - (last_year as f64) / 100000.0_f64) as u128 + LEAP[(month - 1) as usize] as u128 + day as u128) % REPEAT_WEAK_DAY_CYCLE as u128) as u8);
    }

    panic!("Unkonwn error");
}

fn main() {
    println!("{}", day_of_week(322600000, 12, 31));
}
