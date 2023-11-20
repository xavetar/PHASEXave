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

use crate::types::{
    data::{
        zone::{Sign, Zone}
    },
    planets::earth::calendar::{
        constants::{
            seconds::{SECONDS_IN_DAY},
        },
    },
};

pub fn zone_recalc(timezone: Zone, unix_time: &mut u128, day_seconds: u128, era_days: &mut u128) {
    let tz_sec: u128 = timezone.to_seconds();
    if timezone.sign == Sign::Signed && *unix_time < tz_sec {
        panic!("[ERROR]: Overflow, signed timezone override self.unix_time!!")
    } else {
        if timezone.sign == Sign::Signed && tz_sec > 0 {
            // Если секунд в последнем дне больше или равно, чем во временной зоне
            if day_seconds >= tz_sec {
                *unix_time -= tz_sec;
                *unix_time += day_seconds;
            } else if day_seconds < tz_sec {
                *era_days -= (tz_sec - day_seconds).div_ceil(SECONDS_IN_DAY);
                *unix_time -= tz_sec;
                *unix_time += day_seconds;
            }
        } else if timezone.sign == Sign::Unsigned {
            let total_secs: u128 = tz_sec + day_seconds;
            if total_secs < SECONDS_IN_DAY {
                *unix_time += tz_sec;
                *unix_time += day_seconds;
            } else if total_secs >= SECONDS_IN_DAY {
                // Округление в меньшую сторону, если даже была указана отсутствующая временная зона [-12, +14]
                *era_days += total_secs.div_euclid(SECONDS_IN_DAY);
                *unix_time += tz_sec;
                *unix_time += day_seconds;
            }
        }
    }
}