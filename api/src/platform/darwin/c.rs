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

// https://github.com/alexey-lysiuk/macos-sdk/blob/main/MacOSX14.4.sdk/usr/include/mach/arm/kern_return.h#L73
pub type kern_return_t = ::libc::c_int;
// https://github.com/alexey-lysiuk/macos-sdk/blob/main/MacOSX14.4.sdk/usr/include/mach/mach_time.h#L41
pub type mach_timebase_info_t = *mut mach_timebase_info;
// https://github.com/alexey-lysiuk/macos-sdk/blob/main/MacOSX14.4.sdk/usr/include/mach/mach_time.h#L42
pub type mach_timebase_info_data_t = mach_timebase_info;

// https://github.com/alexey-lysiuk/macos-sdk/blob/main/MacOSX14.4.sdk/usr/include/mach/kern_return.h#L72
pub const KERN_SUCCESS: kern_return_t = 0;
// https://github.com/alexey-lysiuk/macos-sdk/blob/main/MacOSX14.4.sdk/usr/include/mach/clock_types.h#L86
pub const NSEC_PER_SEC: ::libc::c_ulonglong = 1000000000;

// https://github.com/alexey-lysiuk/macos-sdk/blob/main/MacOSX14.4.sdk/usr/include/mach/mach_time.h#L36
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_timebase_info {
    pub numer: u32,
    pub denom: u32,
}

extern "C" {
    // https://github.com/alexey-lysiuk/macos-sdk/blob/main/MacOSX14.4.sdk/usr/include/mach/mach_time.h#L53
    pub fn mach_absolute_time() -> u64;
    // https://github.com/alexey-lysiuk/macos-sdk/blob/main/MacOSX14.4.sdk/usr/include/mach/mach_time.h#L46
    pub fn mach_timebase_info(info: mach_timebase_info_t) -> kern_return_t;
}