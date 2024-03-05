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

use super::{
    types::enums::{Modes},
};

use PHASEXave::{
    CalendarView, Date, Week,
    RataDie, Sakamoto, Xavetar,
    constants::{MONTHS_IN_YEAR}
};

fn print_help() {
    println!(
        "Usage: {bin_name} [options]\n\nOptions:

        -h, --help                   Show this help message and exit!

        -y, --year YEAR              Set the year: local system year (default),
                                                   any year - max u128

        -m, --method METHOD          Set the method: 1 - Xavetar - High Precision - Fast,
                                                     2 - Rata Die - High Precision - Fast (default)
                                                     3 - Sakamoto - High Precision - Fast

        -c, --columns COLUMNS        Set the number of columns in row: 3 (default),
                                                                       1-12 range (max)

        -l, --margin MARGIN          Set the margin [top, right, bottom, left]: 0,1,1,1 (default),
                                                                                u8,u8,u8,u8 (max)

        -v, --view VIEW              Set the calendar view: 1 - Julian,
                                                            2 - Gregorian (default)
                                                            3 - Solar

        -f, --filename FILENAME      Save to filename: Calendar.txt (default)

        --mode MODE                  Set the mode: 1 - To file
                                                   2 - To console (default)
        ", bin_name = std::path::Path::new::<String>(
            &std::env::args().nth(0)
            .expect("[ERROR]: Binary path is unknown (print_help)!"))
            .file_name().expect("[ERROR]: Can't unwrap filename (print_help)!")
            .to_str().expect("[ERROR]: Can't convert OsStr to &str filename (print_help)!")
    );
}

pub fn parse_args(year: &mut u128, method: &mut fn(CalendarView, u128, u8, u8) -> Week, columns: &mut u8, margin: &mut [u8; 4], view: &mut CalendarView, mode: &mut Modes, filename: &mut String) {
    let mut args: std::iter::Skip<std::env::Args> = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-y" | "--year" => {
                if let Some(year_str) = args.next() {
                    if let Ok(parsed_year) = year_str.parse::<u128>() {
                        *year = parsed_year;
                    } else {
                        println!("[ERROR]: Value is not a unsigned integer: -y, --year [YEAR]");
                        std::process::exit(0);
                    }
                } else {
                    println!("[ERROR]: Value not provided: -y, --year");
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
                        println!("[ERROR]: Value is not a unsigned integer: -m, --method [METHOD]");
                        std::process::exit(0);
                    }
                } else {
                    println!("[ERROR]: Value not provided: -m, --method");
                    std::process::exit(0);
                }
            }
            "-c" | "--columns" => {
                if let Some(columns_str) = args.next() {
                    if let Ok(parsed_columns) = columns_str.parse::<u8>() {
                        if parsed_columns <= MONTHS_IN_YEAR {
                            *columns = parsed_columns;
                        } else {
                            println!("[ERROR]: Invalid width columns: -c, --columns [1, 12]");
                            std::process::exit(0);
                        }
                    } else {
                        println!("[ERROR]: Value is not a unsigned integer: -c, --columns [COLUMNS]");
                        std::process::exit(0);
                    }
                } else {
                    println!("[ERROR]: Value not provided: -c, --columns");
                    std::process::exit(0);
                }
            }
            "-l" | "--margin" => {
                if let Some(margin_str) = args.next() {
                    let margin_values: Vec<u8> = margin_str
                        .split(',')
                        .map(|x| x.parse::<u8>())
                        .collect::<Result<Vec<u8>, std::num::ParseIntError>>()
                        .expect("[ERROR]: One of value is not a unsigned integer!");

                    if margin_values.len() < margin.len() {
                        println!("[ERROR]: Invalid argument format: -l, --margin [top, right, bottom, left]");
                        std::process::exit(0);
                    } else if margin_values.len() > margin.len() {
                        println!("[ERROR]: Invalid argument format: -l, --margin [top, right, bottom, left]");
                        std::process::exit(0);
                    } else {
                        *margin = [margin_values[0], margin_values[1], margin_values[2], margin_values[3]];
                    }
                } else {
                    println!("[ERROR]: Value not provided: -l, --margins");
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
                        println!("[ERROR]: Value is not a unsigned integer: -v, --view [VIEW]");
                        std::process::exit(0);
                    }
                } else {
                    println!("[ERROR]: Value not provided: -v, --view");
                    std::process::exit(0);
                }
            }
            "-f" | "--filename" => {
                if let Some(filename_str) = args.next() {
                    if let Ok(parsed_filename) = filename_str.parse::<String>() {
                        if parsed_filename.len() >= 1 {
                            *filename = parsed_filename;
                        } else {
                            println!("[ERROR]: Length cannot be less than 1 character");
                        }
                    } else {
                        println!("[ERROR]: Value is not a String: --save [FILENAME]");
                        std::process::exit(0);
                    }
                } else {
                    println!("[ERROR]: Value not provided: --save");
                    std::process::exit(0);
                }
            }
            "--mode" => {
                if let Some(mode_str) = args.next() {
                    if let Ok(parsed_mode) = mode_str.parse::<u8>() {
                        match parsed_mode {
                            1 => *mode = Modes::File,
                            2 => *mode = Modes::Console,
                            _ => {
                                println!("[ERROR]: Invalid mode type: --mode");
                                std::process::exit(0);
                            }
                        }
                    } else {
                        println!("[ERROR]: Value is not a unsigned integer: --mode [MODE]");
                        std::process::exit(0);
                    }
                } else {
                    println!("[ERROR]: Value not provided: --mode");
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