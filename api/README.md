![Greenfield](https://img.shields.io/badge/Greenfield-0fc908.svg)
[![Rust](https://github.com/xavetar/PHASEXave/actions/workflows/build.yml/badge.svg)](https://github.com/xavetar/PHASEXave/actions/workflows/build.yml)
[![Deps](https://deps.rs/repo/github/xavetar/PHASEXave/status.svg)](https://deps.rs/repo/github/xavetar/PHASEXave)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# PHASEXave

![PHASEXave Logo](res/phasexave-header.png)

<div style="display: flex; justify-content: center; gap: 20px;">
    <a href="https://nowpayments.io/donation?api_key=NRH28QG-ABRM7CC-J7NVGXN-F8FTRS1&source=lk_donation&medium=referral" target="_blank">
        <img src="https://nowpayments.io/images/embeds/donation-button-black.svg" alt="Crypto donation button by NOWPayments" style="height: 60px !important; width: 217px !important;">
    </a>
    <a href="https://www.buymeacoffee.com/xavetar" target="_blank">
        <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important; width: 217px !important;">
    </a>
</div>

## Validity of Calendars

- The Gregorian calendar advances every year by 0.000031 days in absolute values, relative to the point of the vernal equinox. Between 3225~3232 the divergence will reach about 2 days from the Vernal Equinox.
- The Solar calendar solves all the leap year problems of the Gregorian and Julian calendar, but it is a bit slower than the others because of more complex computational operations.

## Features

Add feature to Cargo.toml to use local timezone, absolute time functions:

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
    println!("{hours}:{minutes}:{seconds}", hours = time.hours, minutes = time.minutes, seconds = time.seconds);
}
```

#### Now

Get current time with any timezone:

```rust
use PHASEXave::{Time, Sign, Zone};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Signed, hours: 4, minutes: 30, seconds: 0 };
    let time: Time = Time::now(timezone);
    println!("{hours}:{minutes}:{seconds}", hours = time.hours, minutes = time.minutes, seconds = time.seconds);
}
```

#### Local

Get current time with local timezone:

```rust
use PHASEXave::{Time};

fn main() {
    let time: Time = Time::local();
    println!("{hours}:{minutes}:{seconds}", hours = time.hours, minutes = time.minutes, seconds = time.seconds);
}
```

#### Uptime

##### Get uptime (without sleep time is not a boot time) (macOS/Darwin/XNU, Unix/Linux, Windows):

```rust
use PHASEXave::{Time, Uptime};

fn main() {
    let time: Uptime = Time::absolute();
    println!("{hours}:{minutes}:{seconds}", hours = time.hours, minutes = time.minutes, seconds = time.seconds);
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
    println!("{yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Julian

Get UTC date for Julian calendar:

```rust
use PHASEXave::{CalendarView, Date};

fn main() {
    let date: Date = Date::utc(CalendarView::Julian);
    println!("{yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Gregorian

Get UTC date for Gregorian calendar:

```rust
use PHASEXave::{CalendarView, Date};

fn main() {
    let date: Date = Date::utc(CalendarView::Gregorian);
    println!("{yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

#### Now

##### Solar

Get current date with any timezone for Solar calendar:

```rust
use PHASEXave::{CalendarView, Date, Zone, Sign};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 };
    let date: Date = Date::now(CalendarView::Solar, timezone);
    println!("{yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Julian

Get current date with any timezone for Julian calendar:

```rust
use PHASEXave::{CalendarView, Date, Zone, Sign};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 };
    let date: Date = Date::now(CalendarView::Julian, timezone);
    println!("{yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Gregorian

Get current date with any timezone for Gregorian calendar:

```rust
use PHASEXave::{CalendarView, Date, Zone, Sign};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 };
    let date: Date = Date::now(CalendarView::Gregorian, timezone);
    println!("{yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

#### Local

##### Solar

Get current date with local timezone for Solar calendar:

```rust
use PHASEXave::{CalendarView, Date};

fn main() {
    let date: Date = Date::local(CalendarView::Solar);
    println!("{yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Julian

Get current date with local timezone for Julian calendar:

```rust
use PHASEXave::{CalendarView, Date};

fn main() {
    let date: Date = Date::local(CalendarView::Julian);
    println!("{yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Gregorian

Get current date with local timezone for Gregorian calendar:

```rust
use PHASEXave::{CalendarView, Date};

fn main() {
    let date: Date = Date::local(CalendarView::Gregorian);
    println!("{yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

#### Presentation

Convert any* date representation between different calendars:

```rust
use PHASEXave::{CalendarView, Date, Sign, Zone, Julian, Gregorian, Solar};

fn main() {
    let mut date: Date = Date {
        day: 27,
        month: 2,
        year: 3226,
        timezone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Solar
    };
    <Date as Julian>::to_presentation(&mut date, true);
    println!("JL: {yyyy}/{mm}/{dd}:{era_days}:{unix_time}", yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time);
    <Date as Gregorian>::to_presentation(&mut date, true);
    println!("GR: {yyyy}/{mm}/{dd}:{era_days}:{unix_time}", yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time);
    <Date as Solar>::to_presentation(&mut date, true);
    println!("SL: {yyyy}/{mm}/{dd}:{era_days}:{unix_time}", yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time);
}
```

Output:

```
JL: 3226/2/27:1177987:39642393600
GR: 3226/2/27:1177965:39640492800
SL: 3226/2/27:1177964:39640406400
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
        timezone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Solar
    };
    <Date as Julian>::to_date(&mut date, false);
    println!("JL: {yyyy}/{mm}/{dd}:{era_days}:{unix_time}", yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time);
}
```

Output:

```
JL: 3226/2/4:1177964:39640406400
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
        timezone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Solar
    };
    <Date as Gregorian>::to_date(&mut date, false);
    println!("GR: {yyyy}/{mm}/{dd}:{era_days}:{unix_time}", yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time);
}
```

Output:

```
GR: 3226/2/28:1177966:39640579200
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
        timezone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Julian
    };
    <Date as Solar>::to_date(&mut date, false);
    println!("SL: {yyyy}/{mm}/{dd}:{era_days}:{unix_time}", yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time);
}
```

Output:

```
SL: 3226/2/28:1177965:39640492800
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
        timezone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Julian
    };
    <Date as Gregorian>::to_date(&mut date, false);
    println!("GR: {yyyy}/{mm}/{dd}:{era_days}:{unix_time}", yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time);
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
        timezone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Gregorian
    };
    <Date as Solar>::to_date(&mut date, false);
    println!("SL: {yyyy}/{mm}/{dd}:{era_days}:{unix_time}", yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time);
}
```

Output:

```
SL: 3226/3/1:1177966:39640579200
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
        timezone: Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 },
        unix_time: 0,
        era_days: 0,
        view: CalendarView::Gregorian
    };
    <Date as Julian>::to_date(&mut date, false);
    println!("JL: {yyyy}/{mm}/{dd}:{era_days}:{unix_time}", yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time);
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
    let timezone: Zone = Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 };
    let mut date: Date = Date::now(CalendarView::Gregorian, timezone);
    <Date as Solar>::to_date(&mut date, true);
    <Date as Julian>::to_date(&mut date, true);
    <Date as Gregorian>::to_date(&mut date, true);
    println!("GR: {yyyy}/{mm}/{dd}:{era_days}:{unix_time}", yyyy = date.year, mm = date.month, dd = date.day, era_days = date.era_days, unix_time = date.unix_time);
}
```

Output:

```
GR: 2024/3/4:738949:1709574752
```

#### Day of Week

Get week day from Solar, Julian and Gregorian calendar:

##### Solar

Get week day from Date:

```rust
use PHASEXave::{CalendarView, Date, RataDie, Xavetar, Sakamoto, Sign, Zone};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Unsigned, hours: 8, minutes: 0, seconds: 0 };

    let date: Date = Date::now(CalendarView::Solar, timezone);
    println!(
        "Solar Week day:\n\nRata Die: {rata_die}\nXavetar: {xavetar}\nSakamoto: {sakamoto}\n",
        rata_die = <Date as RataDie>::week_day(&date).name(),
        xavetar = <Date as Xavetar>::week_day(&date).name(),
        sakamoto = <Date as Sakamoto>::week_day(&date).name(),
    );
    println!("Date: {yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

Output:

```
Solar Week day:

Rata Die: Tuesday
Xavetar: Tuesday
Sakamoto: Tuesday

Date: 2024/3/6
```

Get week day from any date:

```rust
use PHASEXave::{CalendarView, Date, RataDie, Sakamoto, Xavetar};

fn main() {
    let (yyyy, mm, dd): (u128, u8, u8) = (1582, 10, 5);
    println!(
        "Solar Week day:\n\nRata Die: {rata_die}\nXavetar: {xavetar}\nSakamoto: {sakamoto}\n",
        rata_die = <Date as RataDie>::from(CalendarView::Solar, yyyy, mm, dd).name(),
        xavetar = <Date as Xavetar>::from(CalendarView::Solar, yyyy, mm, dd).name(),
        sakamoto = <Date as Sakamoto>::from(CalendarView::Solar, yyyy, mm, dd).name(),
    );
    println!("Date: {yyyy}/{mm}/{dd}", yyyy = yyyy, mm = mm, dd = dd);
}
```

Output:

```
Solar Week day:

Rata Die: Tuesday
Xavetar: Tuesday
Sakamoto: Tuesday

Date: 1582/10/5
```

##### Julian

Get week day from Date:

```rust
use PHASEXave::{CalendarView, Date, RataDie, Xavetar, Sakamoto, Sign, Zone};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Unsigned, hours: 8, minutes: 0, seconds: 0 };

    let date: Date = Date::now(CalendarView::Julian, timezone);
    println!(
        "Julian Week day:\n\nRata Die: {rata_die}\nXavetar: {xavetar}\nSakamoto: {sakamoto}\n",
        rata_die = <Date as RataDie>::week_day(&date).name(),
        xavetar = <Date as Xavetar>::week_day(&date).name(),
        sakamoto = <Date as Sakamoto>::week_day(&date).name(),
    );
    println!("Date: {yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

Output:

```
Julian Week day:

Rata Die: Tuesday
Xavetar: Tuesday
Sakamoto: Tuesday

Date: 2024/2/21
```

Get week day from any date:

```rust
use PHASEXave::{CalendarView, Date, RataDie, Xavetar, Sakamoto};

fn main() {
    let (yyyy, mm, dd): (u128, u8, u8) = (1582, 10, 5);
    println!(
        "Julian Week day:\n\nRata Die: {rata_die}\nXavetar: {xavetar}\nSakamoto: {sakamoto}\n",
        rata_die = <Date as RataDie>::from(CalendarView::Julian, yyyy, mm, dd).name(),
        xavetar = <Date as Xavetar>::from(CalendarView::Julian, yyyy, mm, dd).name(),
        sakamoto = <Date as Sakamoto>::from(CalendarView::Julian, yyyy, mm, dd).name(),
    );
    println!("Date: {yyyy}/{mm}/{dd}", yyyy = yyyy, mm = mm, dd = dd);
}
```

Output:

```
Julian Week day:

Rata Die: Friday
Xavetar: Friday
Sakamoto: Friday

Date: 1582/10/5
```

##### Gregorian

Get week day from Date:

```rust
use PHASEXave::{CalendarView, Date, RataDie, Xavetar, Sakamoto, Sign, Zone};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Unsigned, hours: 8, minutes: 0, seconds: 0 };

    let date: Date = Date::now(CalendarView::Gregorian, timezone);
    println!(
        "Gregorian Week day:\n\nRata Die: {rata_die}\nSakamoto: {sakamoto}\nXavetar: {xavetar}\n",
        rata_die = <Date as RataDie>::week_day(&date).name(),
        xavetar = <Date as Xavetar>::week_day(&date).name(),
        sakamoto = <Date as Sakamoto>::week_day(&date).name(),
    );
    println!("Date: {yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

Output:

```
Gregorian Week day:

Rata Die: Tuesday
Sakamoto: Tuesday
Xavetar: Tuesday

Date: 2024/3/5
```

Get week day from any date:

```rust
use PHASEXave::{CalendarView, Date, RataDie, Xavetar, Sakamoto};

fn main() {
    let (yyyy, mm, dd): (u128, u8, u8) = (1582, 10, 5);
    println!(
        "Gregorian Week day:\n\nRata Die: {rata_die}\nXavetar: {xavetar}\nSakamoto: {sakamoto}\n",
        rata_die = <Date as RataDie>::from(CalendarView::Gregorian, yyyy, mm, dd).name(),
        xavetar = <Date as Xavetar>::from(CalendarView::Gregorian, yyyy, mm, dd).name(),
        sakamoto = <Date as Sakamoto>::from(CalendarView::Gregorian, yyyy, mm, dd).name(),
    );
    println!("Date: {yyyy}/{mm}/{dd}", yyyy = yyyy, mm = mm, dd = dd);
}
```

Output:

```
Gregorian Week day:

Rata Die: Tuesday
Xavetar: Tuesday
Sakamoto: Tuesday

Date: 1582/10/5
```

## License

PHASEXave is primarily distributed under the terms of three the Anti-Virus license and MIT license and the Apache License (Version 2.0)

See [LICENSE-ANTI-VIRUS](LICENSE-Anti-Virus) and [LICENSE-APACHE](LICENSE-Apache) and [LICENSE-MIT](LICENSE-MIT) for details.
