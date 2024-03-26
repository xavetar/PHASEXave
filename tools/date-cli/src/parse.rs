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

use PHASEXave::{
    CalendarView,
    Date, Week,
    Zone, Sign,
    RataDie, Xavetar, Sakamoto
};

fn print_help() {
    println!(
        "Usage: {bin_name} [options]\n\nOptions:

        -z, --zone [ZONE]              Set the time_zone:[+/-][hours:minutes:seconds]: local zone (default)
                                                                                       [+/-][255-1:255-1:255-1] (max)

        -m, --method [METHOD]          Determining the day of week method: 1 - Xavetar - High Precision - Fast
                                                                           2 - Rata Die - High Precision - Fast (default)
                                                                           3 - Sakamoto - High Precision - Fast

        -v, --view [VIEW]              Set the calendar view: 1 - Julian
                                                              2 - Gregorian (default)
                                                              3 - Solar

        ", bin_name = std::path::Path::new::<String>(
            &std::env::args().nth(0)
                .expect("[ERROR]: Binary path is unknown (print_help)!"))
            .file_name().expect("[ERROR]: Can't unwrap filename (print_help)!")
            .to_str().expect("[ERROR]: Can't convert OsStr to &str filename (print_help)!")
    );
}

pub fn parse_args(time_zone: &mut Zone, method: &mut fn(CalendarView, u64, u8, u8) -> Week, view: &mut CalendarView) {
    let mut args: std::iter::Skip<std::env::Args> = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-z" | "--zone" => {
                if let Some(zone_str) = args.next() {
                    let sign: Sign;

                    if zone_str.starts_with("-") {
                        sign = Sign::Signed
                    } else if zone_str.starts_with("+") {
                        sign = Sign::Unsigned
                    } else {
                        panic!("[ERROR] Invalid sign in time zone: {char:?}", char = zone_str.chars().nth(0));
                    }

                    let zone_values: Vec<u8> = zone_str[1..]
                        .split(':')
                        .map(|x| x.parse::<u8>())
                        .collect::<Result<Vec<u8>, std::num::ParseIntError>>()
                        .expect("[ERROR]: One of value or more is not a unsigned integer or overflow type!");

                    if zone_values.len() < 3 {
                        println!("[ERROR]: Invalid argument format: -z, --zone [+/-][hours:minutes:seconds]");
                        std::process::exit(0);
                    } else if zone_values.len() > 3 {
                        println!("[ERROR]: Invalid argument format: -z, --zone [+/-][hours:minutes:seconds]");
                        std::process::exit(0);
                    } else {
                        (time_zone.sign, time_zone.hours, time_zone.minutes, time_zone.seconds)
                        =
                        (sign, zone_values[0], zone_values[1], zone_values[2]);
                    }
                } else {
                    println!("[ERROR]: Value not provided: -z, --margins");
                    std::process::exit(0);
                }
            }
            "-m" | "--method" => {
                if let Some(method_str) = args.next() {
                    if let Ok(parsed_method) = method_str.parse::<u8>() {
                        match parsed_method {
                            1 => *method = <Date as Xavetar>::from,
                            2 => *method = <Date as RataDie>::from,
                            3 => *method = <Date as Sakamoto>::from,
                            _ => {
                                println!("[ERROR]: Invalid method type: -m, --method");
                                std::process::exit(0);
                            }
                        }
                    } else {
                        println!("[ERROR]: Value is not a unsigned integer or overflow type: -m, --method [METHOD]");
                        std::process::exit(0);
                    }
                } else {
                    println!("[ERROR]: Value not provided: -m, --method");
                    std::process::exit(0);
                }
            }
            "-v" | "--view" => {
                if let Some(view_str) = args.next() {
                    if let Ok(parsed_view) = view_str.parse::<u8>() {
                        match parsed_view {
                            1 => *view = CalendarView::Julian,
                            2 => *view = CalendarView::Gregorian,
                            3 => *view = CalendarView::Solar,
                            _ => {
                                println!("[ERROR]: Invalid view type: -v, --view");
                                std::process::exit(0);
                            }
                        }
                    } else {
                        println!("[ERROR]: Value is not a unsigned integer or overflow type: -v, --view [VIEW]");
                        std::process::exit(0);
                    }
                } else {
                    println!("[ERROR]: Value not provided: -v, --view");
                    std::process::exit(0);
                }
            }
            _ => {
                println!("[ERROR]: Unknown option: {}", arg);
                std::process::exit(0);
            }
        }
    }
}