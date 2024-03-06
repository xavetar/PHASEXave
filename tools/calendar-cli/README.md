![Greenfield](https://img.shields.io/badge/Greenfield-0fc908.svg)
![Build and test](https://img.shields.io/badge/build-passing-brightgreen.svg)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# Calendar-CLI

![Calendar-CLI Logo](../../tools/calendar-cli/res/calendar-cli.svg)

## Command-Line Options

The program accepts the following command-line options:

```
-y, --year [YEAR]: Set the year. The default value is the local system year, but you can also specify any year up to a maximum of u128.
-m, --method [METHOD]: Set the method for determining the day of the week. There are three options available:
    1: Xavetar - High Precision - Fast
    2: Rata Die - High Precision - Fast (default)
    3: Sakamoto - High Precision - Fast
-c, --columns [COLUMNS]: Set the number of columns in a row. The default value is 3, but you can specify a value in the range of 1-12.
-l, --margin [MARGIN]: Set the margin [top, right, bottom, left]. The default value is 0,1,1,1, but you can specify values up to u8,u8,u8,u8.
-v, --view [VIEW]: Set the calendar view. There are two options available:
    1: Julian
    2: Gregorian (default)
    3: Solar
-f, --filename [FILENAME]: Save to filename. The default filename is Calendar.txt, but you can specify a different filename.
--mode [MODE]: Set the mode. There are two options available:
    1: To file
    2: To console (default)
```


## Usage

Generate a Gregorian calendar for the current year (default), with 3 columns (default), using the my algorithm method (default), and display it in the console:

```shell
calendar-cli
```

Generate a Julian calendar for the current year (default), with 3 columns (default), using the my algorithm method (default), and display it in the console:

```shell
calendar-cli -v 1
```

Generate a Gregorian calendar for the current year (default), with 3 columns (default), using the Rata Die method (default), and display it in the console:

```shell
calendar-cli --mode 2
```

Generate a Gregorian calendar for the year 2022, with 2 columns, using the Xavetar method, and display it in the console:

```shell
calendar-cli -y 2022 -c 2 -m 1 --mode 2
```

Generate a Gregorian calendar for the current year (default), with 8 columns, using the Rata Die method (default), and save it to a file named "output.txt" in the current directory:

```shell
calendar-cli -c 8 -f output.txt --mode 1
```

Generate a Gregorian calendar for the year 2023, with 4 columns, using the Xavetar method, and save it to a file named "2023.txt" in the current directory:

```shell
calendar-cli -y 2023 -c 4 -m 1 -f 2023.txt --mode 1
```

Generate a Gregorian calendar for the year 2024, with 6 columns, using the Sakamoto method, and save it to a file named "calendar.txt" in the current directory:

```shell
calendar-cli -y 2024 -c 6 -m 3 -f calendar.txt --mode 1
```

Generate a Gregorian calendar for the year 2030, with 5 columns, using the Rata Die method, and save it to a file named "calendar-2030.txt" in the current directory:

```shell
calendar-cli -y 2030 -c 5 -m 2 -f calendar-2030.txt --mode 1
```

Generate a Julian calendar for the year 2035, with 7 columns, using the Sakamoto method, and save it to a file named "julian-calendar-2035.txt" in the current directory:

```shell
calendar-cli -y 2035 -c 7 -m 3 -f julian-calendar-2035.txt --mode 1 -v 1
```

Generate a Julian calendar for the year 2024, with 2 columns, with margings, using the Xavetar method, and save it to a file named "julian-calendar-2024.txt" in the current directory:

```shell
calendar-cli -y 2024 -c 7 -m 3 -l 0,2,1,2 -f julian-calendar-2024.txt --mode 1 -v 1
```

## License

Calendar-CLI is primarily distributed under the terms of three the Anti-Virus license and MIT license and the Apache License (Version 2.0)

See [LICENSE-ANTI-VIRUS](../../LICENSE-Anti-Virus) and [LICENSE-APACHE](../../LICENSE-Apache) and [LICENSE-MIT](../../LICENSE-MIT) for details.
