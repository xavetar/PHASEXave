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

// Алгоритм для вычисления дня недели (Rata Die):

const BASE_DAYS_YEAR: u16 = 365;
const LEAP_DAYS_YEAR: u16 = BASE_DAYS_YEAR + 1;

const REPEAT_WEAK_DAY_CYCLE: u8 = 7;

const SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR: u8 = 0;

const MONTHS_IN_YEAR: u8 = 12;

const BASE_MONTH_DAYS: [u8; MONTHS_IN_YEAR as usize] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LEAP_MONTH_DAYS: [u8; MONTHS_IN_YEAR as usize] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

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
    let mut days: u128 = day as u128;

    for i in 1..year {
        if !is_leap_year_solar(i) {
            days += BASE_DAYS_YEAR as u128;
        } else {
            days += LEAP_DAYS_YEAR as u128;
        }
    }

    if month > 1 {
        for i in 0..(month - 1) {
            if !is_leap_year_solar(year) {
                days += BASE_MONTH_DAYS[i as usize] as u128;
            } else {
                days += LEAP_MONTH_DAYS[i as usize] as u128;
            }
        }
    }

    let dow: u8 = ((days + SHIFT_BEFORE_FIRST_WEEK_DAY_SOLAR as u128) % (REPEAT_WEAK_DAY_CYCLE as u128)) as u8;

    return week_day(dow);
}

fn main() {
    println!("{}", day_of_week(322600000, 12, 31));
}
