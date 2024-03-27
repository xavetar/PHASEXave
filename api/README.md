![Greenfield](https://img.shields.io/badge/Greenfield-0fc908.svg)
[![CI](https://github.com/xavetar/PHASEXave/actions/workflows/on_tag.yaml/badge.svg)](https://github.com/xavetar/PHASEXave/actions/workflows/on_tag.yaml)
[![Deps](https://deps.rs/repo/github/xavetar/PHASEXave/status.svg)](https://deps.rs/repo/github/xavetar/PHASEXave)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/xavetar/PHASEXave/graph/badge.svg?token=9DNJ5FW8CM)](https://codecov.io/gh/xavetar/PHASEXave)

# PHASEXave

![PHASEXave Logo](res/phasexave-header.png)

<div style="display: flex; justify-content: center; gap: 20px;">
    <a href="https://nowpayments.io/donation?api_key=NRH28QG-ABRM7CC-J7NVGXN-F8FTRS1&source=lk_donation&medium=referral" target="_blank">
        <img src="https://nowpayments.io/images/embeds/donation-button-black.svg" alt="Crypto donation button by NOWPayments" style="height: 60px !important; width: 217px !important;">
    </a>
</div>

## Validity of Calendars

- The Gregorian calendar advances every year by 0.000031 days in absolute values, relative to the point of the vernal equinox. Between 3225~3232 the divergence will reach about 2 days from the Vernal Equinox.
- The Solar calendar solves all the leap year problems of the Gregorian and Julian calendar, but it is a bit slower than the others because of more complex computational operations.

## Validity of Leap Year Algorithm

- Solar year: u128 - 2^64
- Julian year: u128 - 2^64
- Gregorian year: u128 - 2^64

Why **2^64**? **u128 * 366** (upper limit of days in a year) overflows era days and seconds after unix epoch, because of this the temporary is u64, in the next version this type will be **u128** again or more.

## Features

Add feature to Cargo.toml to use local time zone, absolute time functions:

- macOS

```toml
[dependencies]
PHASEXave = { version = "*", features = ["platform_specific_functions_darwin"] }
```

- Unix

```toml
[dependencies]
PHASEXave = { version = "*", features = ["platform_specific_functions_unix"] }
```

- Windows

```toml
[dependencies]
PHASEXave = { version = "*", features = ["platform_specific_functions_windows"] }
```

## Usage

### Time

#### UTC

Get UTC time:

```rust
use PHASEXave::{Time};

fn main() {
    let time: Time = Time::utc();
    println!("{hours:02}:{minutes:02}:{seconds:02}", hours = time.hours, minutes = time.minutes, seconds = time.seconds);
}
```

#### Now

Get current time with any time zone:

```rust
use PHASEXave::{Time, Sign, Zone};

fn main() {
    let time: Time = Time::now(
        Zone { sign: Sign::Signed, hours: 4, minutes: 30, seconds: 0 }
    );
    println!("{hours:02}:{minutes:02}:{seconds:02}", hours = time.hours, minutes = time.minutes, seconds = time.seconds);
}
```

#### Local

Get current time with local time zone:

```rust
use PHASEXave::{Time};

fn main() {
    let time: Time = Time::local();
    println!("{hours:02}:{minutes:02}:{seconds:02}", hours = time.hours, minutes = time.minutes, seconds = time.seconds);
}
```

#### Uptime

##### Get uptime (without sleep time is not a boot time) (macOS/Darwin/XNU, Unix/Linux, Windows):

```rust
use PHASEXave::{Uptime};

fn main() {
    let uptime: Uptime = Uptime::absolute();
    println!(
        "Weeks: {weeks}, Days: {days}\n\
        Hours: {hours:02}, Minutes: {minutes:02}, Seconds: {seconds:02}",
        weeks = uptime.weeks, days = uptime.days,
        hours = uptime.hours, minutes = uptime.minutes, seconds = uptime.seconds
    );
}
```

### Date

#### UTC

##### Solar

Get UTC date for Solar calendar:

```rust
use PHASEXave::{CalendarView, Date};

fn main() {
    let date: Date = Date::utc(CalendarView::Solar);
    println!("{yyyy}/{mm:02}/{dd:02}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Julian

Get UTC date for Julian calendar:

```rust
use PHASEXave::{CalendarView, Date};

fn main() {
    let date: Date = Date::utc(CalendarView::Julian);
    println!("{yyyy}/{mm:02}/{dd:02}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Gregorian

Get UTC date for Gregorian calendar:

```rust
use PHASEXave::{CalendarView, Date};

fn main() {
    let date: Date = Date::utc(CalendarView::Gregorian);
    println!("{yyyy}/{mm:02}/{dd:02}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

#### Now

##### Solar

Get current date with any time zone for Solar calendar:

```rust
use PHASEXave::{CalendarView, Date, Zone, Sign};

fn main() {
    let date: Date = Date::now(
        CalendarView::Solar,
        Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 }
    );
    println!("{yyyy}/{mm:02}/{dd:02}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Julian

Get current date with any time zone for Julian calendar:

```rust
use PHASEXave::{CalendarView, Date, Zone, Sign};

fn main() {
    let date: Date = Date::now(
        CalendarView::Julian,
        Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 }
    );
    println!("{yyyy}/{mm:02}/{dd:02}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Gregorian

Get current date with any time zone for Gregorian calendar:

```rust
use PHASEXave::{CalendarView, Date, Zone, Sign};

fn main() {
    let date: Date = Date::now(
        CalendarView::Gregorian,
        Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 }
    );
    println!("{yyyy}/{mm:02}/{dd:02}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

#### Local

##### Solar

Get current date with local time zone for Solar calendar:

```rust
use PHASEXave::{CalendarView, Date};

fn main() {
    let date: Date = Date::local(CalendarView::Solar);
    println!("{yyyy}/{mm:02}/{dd:02}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Julian

Get current date with local time zone for Julian calendar:

```rust
use PHASEXave::{CalendarView, Date};

fn main() {
    let date: Date = Date::local(CalendarView::Julian);
    println!("{yyyy}/{mm:02}/{dd:02}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Gregorian

Get current date with local time zone for Gregorian calendar:

```rust
use PHASEXave::{CalendarView, Date};

fn main() {
    let date: Date = Date::local(CalendarView::Gregorian);
    println!("{yyyy}/{mm:02}/{dd:02}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

#### Presentation

Convert any* date representation between different calendars:

```rust
use PHASEXave::{CalendarView, Date, Sign, Zone, Julian, Gregorian, Solar};

fn main() {
    let date: Date = Date {
        day: 27,
        month: 2,
        year: 3226,
        time_zone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Solar
    };

    let (mut jld, mut grd, mut sld): (Date, Date, Date)
    =
    (date.clone(), date.clone(), date.clone());

    <Date as Julian>::to_presentation(&mut jld, true);
    <Date as Gregorian>::to_presentation(&mut sld, true);
    <Date as Solar>::to_presentation(&mut grd, true);

    println!(
        "JL: {jl_yyyy}/{jl_mm:02}/{jl_dd:02}:{jl_era_days}:{jl_unix_time}\n\
        GR: {gr_yyyy}/{gr_mm:02}/{gr_dd:02}:{gr_era_days}:{gr_unix_time}\n\
        SL: {sl_yyyy}/{sl_mm:02}/{sl_dd:02}:{sl_era_days}:{sl_unix_time}",
        jl_yyyy = jld.year, jl_mm = jld.month, jl_dd = jld.day, jl_era_days = jld.era_days, jl_unix_time = jld.unix_time,
        gr_yyyy = grd.year, gr_mm = grd.month, gr_dd = grd.day, gr_era_days = grd.era_days, gr_unix_time = grd.unix_time,
        sl_yyyy = sld.year, sl_mm = sld.month, sl_dd = sld.day, sl_era_days = sld.era_days, sl_unix_time = sld.unix_time
    );
}
```

Output:

```
JL: 3226/02/27:1177987:39642393600
GR: 3226/02/27:1177964:39640406400
SL: 3226/02/27:1177965:39640492800
```

\* - any, but except first BCE days in Julian Calendar,

#### Conversion

##### Solar to Julian

Convert any Solar date to Julian date:

```rust
use PHASEXave::{Date, Sign, Zone, Julian, CalendarView};

fn main() {
    let mut date: Date = Date {
        day: 27,
        month: 2,
        year: 3226,
        time_zone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Solar
    };
    <Date as Julian>::to_date(&mut date, false);
    println!(
        "JL: {yyyy}/{mm:02}/{dd:02}:{era_days}:{unix_time}",
        yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time
    );
}
```

Output:

```
JL: 3226/02/04:1177964:39640406400
```

##### Solar to Gregorian

Convert any Solar date to Gregorian date:

```rust
use PHASEXave::{Date, Sign, Zone, Gregorian, CalendarView};

fn main() {
    let mut date: Date = Date {
        day: 1,
        month: 3,
        year: 3226,
        time_zone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Solar
    };
    <Date as Gregorian>::to_date(&mut date, false);
    println!(
        "GR: {yyyy}/{mm:02}/{dd:02}:{era_days}:{unix_time}",
        yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time
    );
}
```

Output:

```
GR: 3226/02/28:1177966:39640579200
```

##### Julian to Solar

Convert *any Julian date to Solar date:

```rust
use PHASEXave::{Date, Sign, Zone, Solar, CalendarView};

fn main() {
    let mut date: Date = Date {
        day: 5,
        month: 2,
        year: 3226,
        time_zone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Julian
    };
    <Date as Solar>::to_date(&mut date, false);
    println!(
        "SL: {yyyy}/{mm:02}/{dd:02}:{era_days}:{unix_time}",
        yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time
    );
}
```

Output:

```
SL: 3226/02/28:1177965:39640492800
```

##### Julian to Gregorian

Convert *any Julian date to Gregorian date:

```rust
use PHASEXave::{Date, Sign, Zone, Gregorian, CalendarView};

fn main() {
    let mut date: Date = Date {
        day: 30,
        month: 10,
        year: 2023,
        time_zone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Julian
    };
    <Date as Gregorian>::to_date(&mut date, false);
    println!(
        "GR: {yyyy}/{mm:02}/{dd:02}:{era_days}:{unix_time}",
        yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time
    );
}
```

Output:

```
GR: 2023/11/12:738836:1699747200
```

\* - any, but except first two days, that missing in Gregorian calendar

##### Gregorian to Solar

Convert any Gregorian date to Solar date:

```rust
use PHASEXave::{Date, Sign, Zone, Solar, CalendarView};

fn main() {
    let mut date: Date = Date {
        day: 28,
        month: 2,
        year: 3226,
        time_zone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Gregorian
    };
    <Date as Solar>::to_date(&mut date, false);
    println!(
        "SL: {yyyy}/{mm:02}/{dd:02}:{era_days}:{unix_time}",
        yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time
    );
}
```

Output:

```
SL: 3226/03/01:1177966:39640579200
```

##### Gregorian to Julian

Convert any Gregorian date to Julian date:

```rust
use PHASEXave::{Date, Sign, Zone, Julian, CalendarView};

fn main() {
    let mut date: Date = Date {
        day: 12,
        month: 11,
        year: 2023,
        time_zone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Gregorian
    };
    <Date as Julian>::to_date(&mut date, false);
    println!(
        "JL: {yyyy}/{mm:02}/{dd:02}:{era_days}:{unix_time}",
        yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time
    );
}
```

Output:

```
JL: 2023/10/30:738836:1699747200
```

##### Between

Convert any Gregorian to Julian and Julian to Gregorian:

```rust
use PHASEXave::{CalendarView, Date, Julian, Gregorian, Solar, Sign, Zone};

fn main() {
    let mut date: Date = Date::now(
        CalendarView::Gregorian,
        Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 }
    );
    <Date as Solar>::to_date(&mut date, true);
    <Date as Julian>::to_date(&mut date, true);
    <Date as Gregorian>::to_date(&mut date, true);
    println!(
        "GR: {yyyy}/{mm:02}/{dd:02}:{era_days}:{unix_time}",
        yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time
    );
}
```

Output:

```
GR: 2024/03/04:738949:1709574752
```

#### Day of Week

Get week day from Solar, Julian and Gregorian calendar:

##### Solar

Get week day from Date:

```rust
use PHASEXave::{CalendarView, Date, RataDie, Xavetar, Sakamoto, Sign, Zone};

fn main() {
    let date: Date = Date::now(
        CalendarView::Solar,
        Zone { sign: Sign::Unsigned, hours: 8, minutes: 0, seconds: 0 }
    );
    println!(
        "Solar Week day:\n\nRata Die: {rata_die}\nXavetar: {xavetar}\nSakamoto: {sakamoto}\n",
        rata_die = <Date as RataDie>::week_day(&date).name(),
        xavetar = <Date as Xavetar>::week_day(&date).name(),
        sakamoto = <Date as Sakamoto>::week_day(&date).name(),
    );
    println!("Date: {yyyy}/{mm:02}/{dd:02}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

Output:

```
Solar Week day:

Rata Die: Tuesday
Xavetar: Tuesday
Sakamoto: Tuesday

Date: 2024/03/06
```

Get week day from any date:

```rust
use PHASEXave::{CalendarView, Date, RataDie, Sakamoto, Xavetar};

fn main() {
    let (yyyy, mm, dd): (u64, u8, u8) = (1582, 10, 5);
    println!(
        "Solar Week day:\n\nRata Die: {rata_die}\nXavetar: {xavetar}\nSakamoto: {sakamoto}\n",
        rata_die = <Date as RataDie>::from(CalendarView::Solar, yyyy, mm, dd).name(),
        xavetar = <Date as Xavetar>::from(CalendarView::Solar, yyyy, mm, dd).name(),
        sakamoto = <Date as Sakamoto>::from(CalendarView::Solar, yyyy, mm, dd).name(),
    );
    println!("Date: {yyyy}/{mm:02}/{dd:02}", yyyy = yyyy, mm = mm, dd = dd);
}
```

Output:

```
Solar Week day:

Rata Die: Tuesday
Xavetar: Tuesday
Sakamoto: Tuesday

Date: 1582/10/05
```

##### Julian

Get week day from Date:

```rust
use PHASEXave::{CalendarView, Date, RataDie, Xavetar, Sakamoto, Sign, Zone};

fn main() {
    let date: Date = Date::now(
        CalendarView::Julian,
        Zone { sign: Sign::Unsigned, hours: 8, minutes: 0, seconds: 0 }
    );
    println!(
        "Julian Week day:\n\nRata Die: {rata_die}\nXavetar: {xavetar}\nSakamoto: {sakamoto}\n",
        rata_die = <Date as RataDie>::week_day(&date).name(),
        xavetar = <Date as Xavetar>::week_day(&date).name(),
        sakamoto = <Date as Sakamoto>::week_day(&date).name(),
    );
    println!("Date: {yyyy}/{mm:02}/{dd:02}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

Output:

```
Julian Week day:

Rata Die: Tuesday
Xavetar: Tuesday
Sakamoto: Tuesday

Date: 2024/02/21
```

Get week day from any date:

```rust
use PHASEXave::{CalendarView, Date, RataDie, Xavetar, Sakamoto};

fn main() {
    let (yyyy, mm, dd): (u64, u8, u8) = (1582, 10, 5);
    println!(
        "Julian Week day:\n\nRata Die: {rata_die}\nXavetar: {xavetar}\nSakamoto: {sakamoto}\n",
        rata_die = <Date as RataDie>::from(CalendarView::Julian, yyyy, mm, dd).name(),
        xavetar = <Date as Xavetar>::from(CalendarView::Julian, yyyy, mm, dd).name(),
        sakamoto = <Date as Sakamoto>::from(CalendarView::Julian, yyyy, mm, dd).name(),
    );
    println!("Date: {yyyy}/{mm:02}/{dd:02}", yyyy = yyyy, mm = mm, dd = dd);
}
```

Output:

```
Julian Week day:

Rata Die: Friday
Xavetar: Friday
Sakamoto: Friday

Date: 1582/10/05
```

##### Gregorian

Get week day from Date:

```rust
use PHASEXave::{CalendarView, Date, RataDie, Xavetar, Sakamoto, Sign, Zone};

fn main() {
    let date: Date = Date::now(
        CalendarView::Gregorian,
        Zone { sign: Sign::Unsigned, hours: 8, minutes: 0, seconds: 0 }
    );
    println!(
        "Gregorian Week day:\n\nRata Die: {rata_die}\nSakamoto: {sakamoto}\nXavetar: {xavetar}\n",
        rata_die = <Date as RataDie>::week_day(&date).name(),
        xavetar = <Date as Xavetar>::week_day(&date).name(),
        sakamoto = <Date as Sakamoto>::week_day(&date).name(),
    );
    println!("Date: {yyyy}/{mm:02}/{dd:02}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

Output:

```
Gregorian Week day:

Rata Die: Tuesday
Sakamoto: Tuesday
Xavetar: Tuesday

Date: 2024/03/05
```

Get week day from any date:

```rust
use PHASEXave::{CalendarView, Date, RataDie, Xavetar, Sakamoto};

fn main() {
    let (yyyy, mm, dd): (u64, u8, u8) = (1582, 10, 5);
    println!(
        "Gregorian Week day:\n\nRata Die: {rata_die}\nXavetar: {xavetar}\nSakamoto: {sakamoto}\n",
        rata_die = <Date as RataDie>::from(CalendarView::Gregorian, yyyy, mm, dd).name(),
        xavetar = <Date as Xavetar>::from(CalendarView::Gregorian, yyyy, mm, dd).name(),
        sakamoto = <Date as Sakamoto>::from(CalendarView::Gregorian, yyyy, mm, dd).name(),
    );
    println!("Date: {yyyy}/{mm:02}/{dd:02}", yyyy = yyyy, mm = mm, dd = dd);
}
```

Output:

```
Gregorian Week day:

Rata Die: Tuesday
Xavetar: Tuesday
Sakamoto: Tuesday

Date: 1582/10/05
```

## License

PHASEXave is primarily distributed under the terms of three the Anti-Virus license and MIT license and the Apache License (Version 2.0)

See [LICENSE-ANTI-VIRUS](LICENSE) and [LICENSE-APACHE](LICENSE) and [LICENSE-MIT](LICENSE) for details.