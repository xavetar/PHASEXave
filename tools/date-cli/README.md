![Greenfield](https://img.shields.io/badge/Greenfield-0fc908.svg)
![Build and test](https://img.shields.io/badge/build-passing-brightgreen.svg)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# Date-CLI

![Calendar-CLI Logo](../../tools/date-cli/res/date-cli.svg)

## Command-Line Options

The program accepts the following command-line options:


    -z, --zone [ZONE]              Set the timezone. The default value is local system timezone.
    -m, --method [METHOD]: Set the method for determining the day of the week. There are three options available:
        1: Xavetar - High Precision - Fast
        2: Rata Die - High Precision - Fast (default)
        3: Sakamoto - High Precision - Fast
    -v, --view [VIEW]: Set the calendar view. There are two options available:
        1: Julian
        2: Gregorian (default)
        3: Solar

## Usage

Get current date and time with local timezone for Gregorian calendar:

```bash
date-cli
```

Get current date and time with local timezone for Julian calendar:


```bash
date-cli -v 1
```

Get current date and time with local timezone for Solar calendar:


```bash
date-cli -v 3
```

Get current date with your timezone for Gregorian calendar with Sakamoto method to determining the day of the week:

```bash
date-cli -z -06:00:00 -m 3 -v 2
```

Get current date with your timezone for Julian calendar with Xavetar method to determining the day of the week:

```bash
date-cli -z +12:00:00 -m 1 -v 1
```

Get current date with your timezone for Solar calendar with Rata Die method to determining the day of the week:

```bash
date-cli -z +03:00:00 -m 2 -v 3
```

## License

Date-CLI is primarily distributed under the terms of three the Anti-Virus license and MIT license and the Apache License (Version 2.0)

See [LICENSE-ANTI-VIRUS](../../LICENSE-Anti-Virus) and [LICENSE-APACHE](../../LICENSE-Apache) and [LICENSE-MIT](../../LICENSE-MIT) for details.
