/*
 * Copyright 2023 Stanislav Mikhailov (xavetar)
 *
 * Licensed under the Creative Commons Zero v1.0 Universal (CC0) License.
 * You may obtain a copy of the License at
 *
 *     http://creativecommons.org/publicdomain/zero/1.0/
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the CC0 license is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
 
const START_YEAR: u16 = 1970;
const BASE_YEAR_DAYS: u16 = 365;
const LEAP_YEAR_DAYS: u16 = 366;

const BASE_MONTH_DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

const LEAP_MONTH_DAYS: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

const fn is_leap_year_gregorian(year: u16) -> bool {
    return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
}
 
const fn current_year(unix_time: u64) -> (u16, u16) {
    let mut year: u16 = START_YEAR;

    let mut total_days: u128 = (((unix_time / 60) / 60) / 24) as u128;

    while (total_days >= (BASE_YEAR_DAYS as u128)) && (total_days != 0) {
        if is_leap_year_gregorian(year) {
            total_days -= LEAP_YEAR_DAYS as u128;
        } else {
            total_days -= BASE_YEAR_DAYS as u128;
        }
        year += 1;
    };

    return (year, total_days as u16);
}

const fn current_month_and_day(current_year: u16, mut days_in_current_year: u16) -> (u8, u8) {
    let months: &[u8; 12];

    if !is_leap_year_gregorian(current_year) {
        months = &BASE_MONTH_DAYS
    } else {
        months = &LEAP_MONTH_DAYS
    }

    let mut current_month: usize = 0;

    loop {
        if days_in_current_year >= (months[current_month] as u16) {
            days_in_current_year -= months[current_month] as u16;
            current_month += 1;
        } else {
            break;
        }
    }
    // (Current month, Current Day)
    return ((current_month + 1) as u8, (days_in_current_year + 1) as u8);
}

const fn get_current_date(unix_time: u64) -> (u8, u8, u16) {
    let (current_year, days_in_current_year) = current_year(unix_time);

    let (current_month, current_day) = current_month_and_day(current_year, days_in_current_year);

    return (current_day, current_month, current_year);
}

const fn utc_time(unix_time: u64) -> (u8, u8, u8) {
    let hours: u8 = ((((unix_time % 86400) / 60) / 60)) as u8;
    let minutes: u8 = ((unix_time % 3600) / 60) as u8;
    let seconds: u8 = (unix_time % 60) as u8;

    return (hours, minutes, seconds);
}

fn day_of_week(current_year: u16, current_month: u8, current_day: u8) -> &'static str {
    let (d, m, y) = if current_month < 3 {
        (current_day, current_month + 12, current_year - 1)
    } else {
        (current_day, current_month, current_year)
    };

    let f: u32 = y as u32 / 100;
    let l: u32 = y as u32 - (100 * f);
    let h: u32 = ((2.6 * m as f32 - 5.39) as u32 + (l / 4) as u32 + (f / 4) as u32 + d as u32 + l as u32 - 2 * f as u32) % 7;

    return match h {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        0 => "Sunday",
        _ => panic!("Invalid day"),
    };
}

fn main() {
    let unix_time: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Error calling SystemTime::now()").as_secs();

    let start: std::time::Instant = std::time::Instant::now();

    let (current_day, current_month, current_year) = get_current_date(unix_time);

    let elapsed: core::time::Duration = start.elapsed();

    let nanoseconds: u128 = elapsed.as_nanos();
    let milliseconds: u128 = elapsed.as_millis();
    let seconds: u64 = elapsed.as_secs();

    println!("Current date: {}", format!("{}/{}/{}", current_day, current_month, current_year));
    println!("Speed: {} nanoseconds/{} milliseconds/{} seconds", nanoseconds, milliseconds, seconds);

    println!("Day of week: {}", day_of_week(current_year, current_month, current_day));

    let (timezone, time_format) = (3 as u8, 24 as u8);

    let (hours, minutes, seconds) = utc_time(unix_time);

    println!("Current time (GMT+3): {:02}:{:02}:{:02}", ((hours + timezone) % time_format), minutes, seconds);
}