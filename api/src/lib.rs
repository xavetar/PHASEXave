/*
 * Copyright 2024 Stanislav Mikhailov (xavetar)
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

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub(crate) mod interface;
pub(crate) mod platform;
pub(crate) mod types;

pub use interface::{Date};
pub use interface::{Time, Uptime};

pub use crate::types::{
    data::zone::{Sign, Zone},
    planets::earth::calendar::{
        view::{CalendarView},
        traits::{
            converter::{Julian, Gregorian, Solar},
            day_of_week::{Xavetar, RataDie, Sakamoto}
        },
        constants::{
            week::{Week},
            months::{Months}
        }
    },
};

pub mod constants {
    pub use crate::types::{
        planets::earth::calendar::{
            constants::{
                week::{DAYS_IN_WEEK},
                months::{MONTHS_IN_YEAR},
            }
        }
    };
}

pub mod functions {
    pub use crate::types::{
        planets::earth::calendar::{
            functions::{
                is_leap_year
            }
        }
    };
}