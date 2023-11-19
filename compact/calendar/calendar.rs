use std::io::Write;

use PHASEXave::{
    CalendarView, Date, Months, Xavetar,
    functions::{
        is_leap_year_julian,
        is_leap_year_gregorian
    }
};

const DAYS_IN_WEEK: u8 = 7;
const MONTHS_IN_YEAR: u8 = 12;
const MAX_DAYS_IN_MONTH: u8 = 31;

const MAX_SHIFT_DAYS_IN_WEEK: u8 = DAYS_IN_WEEK - 1;

const COLUMNS_MATRIX_MONTH: u8 = DAYS_IN_WEEK;
const ROWS_MATRIX_MONTH: u8 = (MAX_SHIFT_DAYS_IN_WEEK + MAX_DAYS_IN_MONTH).div_ceil(DAYS_IN_WEEK);

const DAYS_MATRIX_MONTH: u8 = COLUMNS_MATRIX_MONTH * ROWS_MATRIX_MONTH;

const CALENDAR_YEAR: u128 = 203000000000;

// Margin: [top, right, bottom, left]

const DAY_WIDTH: u8 = 2;
const DAY_FIELD_WIDTH: u8 = DAY_WIDTH + 1;

const LINE_WIDTH: u8 = DAY_FIELD_WIDTH * DAYS_IN_WEEK;
const LINE_HEIGTH: u8 = ROWS_MATRIX_MONTH + 1 + 1 + 1; // rows + week names + space + header

fn to_chars<T: Default + Copy + std::ops::IndexMut<usize, Output = char>>(cursor: &mut usize, text: String) -> T {
    let mut line: T = Default::default();

    for (i, c) in text.chars().enumerate() {
        if c != '\0' {
            line[i] = c;
        }
    }

    *cursor += 1;

    return line;
}


// 2D Array
type DAYS_MATRIX_REPRESENTATION = [[u8; DAYS_MATRIX_MONTH as usize]; MONTHS_IN_YEAR as usize];

// 2D Array
type MONTH_REPRESENTATION = [[char; LINE_WIDTH as usize]; (LINE_HEIGTH) as usize];
// 3D Array
type YEAR_REPRESENTATION = [[[char; LINE_WIDTH as usize]; (LINE_HEIGTH) as usize]; MONTHS_IN_YEAR as usize];

fn format_months(calendar: &DAYS_MATRIX_REPRESENTATION) -> YEAR_REPRESENTATION {
    let mut year_representation: YEAR_REPRESENTATION =  [[['0'; LINE_WIDTH as usize]; (LINE_HEIGTH) as usize]; MONTHS_IN_YEAR as usize];

    for (month, matrix) in calendar.iter().enumerate() {
        let mut cursor_line: usize = 0;

        let mut month_representation: MONTH_REPRESENTATION = [['0'; LINE_WIDTH as usize]; (LINE_HEIGTH) as usize];

        month_representation[cursor_line - 1] = to_chars::<[char; LINE_WIDTH as usize]>(&mut cursor_line, format!("{:^width$}", Months::from((month + 1) as u8).name(), width = LINE_WIDTH as usize));

        month_representation[cursor_line - 1] = to_chars::<[char; LINE_WIDTH as usize]>(&mut cursor_line, format!("{:^width$}", "", width = LINE_WIDTH as usize));

        month_representation[cursor_line - 1] = to_chars::<[char; LINE_WIDTH as usize]>(&mut cursor_line, format!("{:^width$}", "Mo Tu We Th Fr Sa Su", width = LINE_WIDTH as usize));

        let mut line: String = String::from("");

        for (i, field) in matrix.iter().enumerate() {
            if *field == 0 {
                line.push_str("   ");
            } else {
                line.push_str(&format!("{:2} ", field));
            }

            if ((i + 1) as u8 % DAYS_IN_WEEK) == 0 && (line.len() as u8) == LINE_WIDTH {
                month_representation[cursor_line - 1] = to_chars::<[char; LINE_WIDTH as usize]>(&mut cursor_line, format!("{:^21}", line));
                line.clear();
            }
        }

        year_representation[month] = month_representation;
    }

    return year_representation;
}

fn format_year(columns: u8, margin: [u8; 4], calendar: YEAR_REPRESENTATION) -> Vec<Vec<char>> {
    let mut cursor_line: usize = 0;

    let row_objects: u8 = MONTHS_IN_YEAR.div_ceil(columns);

    let WIDTH_CALENDAR: u16 = (LINE_WIDTH as u16 * columns as u16) + ((margin[1] as u16 + margin[3] as u16) * (columns as u16));
    let HEIGHT_CALENDAR: u16 = (LINE_HEIGTH as u16 * row_objects as u16) + ((margin[0] as u16 + margin[2] as u16) * (row_objects as u16)) + 1 + 1; // calendar height + space + year (header)

    let mut calendar_year_representation: Vec<Vec<char>> = Vec::<Vec<char>>::with_capacity(HEIGHT_CALENDAR as usize);

    for _ in 0..HEIGHT_CALENDAR {
        calendar_year_representation.push(vec!['0'; WIDTH_CALENDAR as usize]);
    }

    calendar_year_representation[cursor_line] = format!("{:^width$}", CALENDAR_YEAR, width = WIDTH_CALENDAR as usize).chars().collect::<Vec<char>>(); cursor_line += 1; // Calendar year
    calendar_year_representation[cursor_line] = format!("{:^width$}", "", width = WIDTH_CALENDAR as usize).chars().collect::<Vec<char>>(); cursor_line += 1; // Space

    let mut month_index: u8 = 0;

    for _ in 0..row_objects {
        // Margin top
        for _ in 0..margin[0] {
            calendar_year_representation[cursor_line] = format!("{:^width$}", "", width = WIDTH_CALENDAR as usize).chars().collect::<Vec<char>>(); cursor_line += 1;
        }

        if month_index < MONTHS_IN_YEAR {
            for line in 0..LINE_HEIGTH {
                calendar_year_representation[cursor_line].clear();
                for column in 0..columns {
                    if (month_index + column) < MONTHS_IN_YEAR {
                        calendar_year_representation[cursor_line].append(format!("{:^width$}", "", width = margin[3] as usize).chars().collect::<Vec<char>>().as_mut());
                        calendar_year_representation[cursor_line].append(calendar[(month_index + column) as usize][line as usize].into_iter().collect::<Vec<char>>().as_mut());
                        calendar_year_representation[cursor_line].append(format!("{:^width$}", "", width = margin[1] as usize).chars().collect::<Vec<char>>().as_mut());
                    }
                }
                cursor_line += 1;
            }
            month_index += columns;
        }

        // Margin Bottom
        for _ in 0..margin[2] {
            calendar_year_representation[cursor_line] = format!("{:^width$}", "", width = WIDTH_CALENDAR as usize).chars().collect::<Vec<char>>(); cursor_line += 1;
        }
    }

    return calendar_year_representation;
}

fn save_vec_to_file(calendar_year_representation: Vec<Vec<char>>) {
    let mut calendar: String = calendar_year_representation
        .iter()
        .map(|month| String::from_iter(month) + "\n")
        .collect();

    if calendar.ends_with('\n') {
        calendar.pop();
    }

    if let Ok(mut file) = std::fs::File::create("calendar.txt") {
        if let Err(err) = file.write_all(calendar.as_bytes()) {
            panic!("Failed to write to file: {}!", err);
        }
    } else {
        panic!("Failed to create file!");
    }
}

fn main() {
    let is_leap_year: bool = is_leap_year_gregorian(CALENDAR_YEAR);

    // 2D Array
    let mut calendar: DAYS_MATRIX_REPRESENTATION = [[0; DAYS_MATRIX_MONTH as usize]; MONTHS_IN_YEAR as usize];

    for month_in_year in 1..(MONTHS_IN_YEAR + 1) {
        let month: Months = Months::from(month_in_year);
        let shift: u8 = <Date as Xavetar>::from(CalendarView::Gregorian, CALENDAR_YEAR, month.index(), 1).index() - 1;
        for day_in_month in 1..(month.days(is_leap_year) + 1) {
            calendar[(month_in_year - 1) as usize][(shift + (day_in_month - 1)) as usize] = day_in_month;
        }
    }

    let calendar_year_representation: Vec<Vec<char>> = format_year(4, [0, 2, 2, 2], format_months(&calendar));

    // To console
    for month in calendar_year_representation.clone() {
        println!("{}", String::from_iter(month));
    }

    // To file
    save_vec_to_file(calendar_year_representation);
}
