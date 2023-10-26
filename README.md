![Build and test](https://img.shields.io/badge/build-passing-brightgreen.svg)
[![Deps](https://deps.rs/repo/github/xavetar/PHASEXave/status.svg)](https://deps.rs/repo/github/xavetar/PHASEXave)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# PHASEXave

![PHASEXave Logo](res/logo-header.png)

## Installation
The library is hosted on [crates.io](https://crates.io/crates/PHASEXave/).

```toml
[dependencies]
PHASEXave = "*"
```

## Usage

### Time

Get current time with timezone:

```rust
use PHASEXave::{Time, Sign, Zone};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Signed, hours: 4, minutes: 30, seconds: 0 };
    let time: Time = Time::now(timezone);
    println!("{hours}:{minutes}:{seconds}", hours = time.hours, minutes = time.minutes, seconds = time.seconds);
}
```

### Uptime

Add feature to Cargo.toml:

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

##### Get uptime (without sleep time is not a boot time) (macOS/Darwin/XNU, Unix/Linux, Windows):

```rust
use PHASEXave::{Time, Uptime};

fn main() {
    let time: Uptime = Time::absolute();
    println!("{hours}:{minutes}:{seconds}", hours = time.hours, minutes = time.minutes, seconds = time.seconds);
}
```

### Date

#### Now

```rust
use PHASEXave::{Date, Sign, Zone};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Unsigned, hours: 0, minutes: 0, seconds: 0 };
    let date: Date = Date::now(timezone);
    println!("{yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

#### Conversion

##### Gregorian to Julian

Convert any Gregorian date to Julian date:

```rust
use PHASEXave::{Date, Julian, Sign, Zone};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Unsigned, hours: 3, minutes: 0, seconds: 0 };
    let mut date: Date = Date::now(timezone);
    <Date as Julian>::to_julian(&mut date);
    println!("{yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

##### Julian to Gregorian

Convert any Julian date to Gregorian date:

```rust
use PHASEXave::{Date, Julian, Gregorian, Sign, Zone};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Unsigned, hours: 3, minutes: 0, seconds: 0 };
    let mut date: Date = Date::now(timezone);
    <Date as Julian>::to_julian(&mut date);
    <Date as Gregorian>::to_gregorian(&mut date);
    println!("{yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

* any, but except first two days, that missing in Gregorian calendar

#### Day of Week

Get week day from Gregorian and Julian calendar:

##### Gregorian

Get week day from Date:

```rust
use PHASEXave::{Date, RataDie, Xavetar, Sakamoto, Sign, Zone};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Unsigned, hours: 8, minutes: 0, seconds: 0 };

    let date: Date = Date::now(timezone);
    println!(
        "Gregorian Week day:\n\nRata Die: {rata_die}\nSakamoto: {sakamoto}\nXavetar: {xavetar}\n",
        rata_die = <Date as RataDie>::week_day(&date).name(),
        xavetar = <Date as Xavetar>::week_day(&date).name(),
        sakamoto = <Date as Sakamoto>::week_day(&date).name(),
    );
    println!("Date: {yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

Get week day from any date:

```rust
use PHASEXave::{CalendarView, Date, Julian, RataDie, Xavetar, Sakamoto};

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

##### Julian

Get week day from Date:

```rust
use PHASEXave::{Date, Julian, RataDie, Xavetar, Sakamoto, Sign, Zone};

fn main() {
    let timezone: Zone = Zone { sign: Sign::Unsigned, hours: 8, minutes: 0, seconds: 0 };

    let mut date: Date = Date::now(timezone);
    <Date as Julian>::to_julian(&mut date);
    println!(
        "Julian Week day:\n\nRata Die: {rata_die}\nXavetar: {xavetar}\nSakamoto: {sakamoto}\n",
        rata_die = <Date as RataDie>::week_day(&date).name(),
        xavetar = <Date as Xavetar>::week_day(&date).name(),
        sakamoto = <Date as Sakamoto>::week_day(&date).name(),
    );
    println!("Date: {yyyy}/{mm}/{dd}", yyyy = date.year, mm = date.month, dd = date.day);
}
```

Get week day from any date:

```rust
use PHASEXave::{CalendarView, Date, Julian, RataDie, Xavetar, Sakamoto};

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

## License

PHASEXave is primarily distributed under the terms of three the Anti-Virus license and MIT license and the Apache License (Version 2.0)

See [LICENSE-ANTI-VIRUS](LICENSE-Anti-Virus) and [LICENSE-APACHE](LICENSE-Apache) and [LICENSE-MIT](LICENSE-MIT) for details.
