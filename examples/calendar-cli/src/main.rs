use calendar_cli::{
    text::{
        format_months_by_days, format_months_to_text,
        format_calendar_from_text_months
    },
    save::{save_to_file},
    parse::{parse_args},
    types::enums::{Modes}
};

use PHASEXave::{
    CalendarView, Date, Week,
    RataDie,
    functions::{is_leap_year}
};

fn make_calendar(view: CalendarView, method: fn(CalendarView, u128, u8, u8) -> Week, year: u128, columns: u8, margin: [u8; 4]) -> Vec<Vec<char>> {
    let is_leap_year: bool = is_leap_year(view, year);

    return format_calendar_from_text_months(year, columns, margin, format_months_to_text(&format_months_by_days(view, method, year, is_leap_year)));
}

fn main() {
    let (
        mut year,
        mut method,
        mut columns,
        mut margin,
        mut view,
        mut mode,
        mut filename
    )
    :
    (
        u128,
        fn(CalendarView, u128, u8, u8) -> Week,
        u8,
        [u8; 4],
        CalendarView,
        Modes,
        String
    )
    =
    (
        Date::local(CalendarView::Gregorian).year,
        <Date as RataDie>::from,
        3,
        [0, 1, 1, 1],
        CalendarView::Gregorian,
        Modes::Console,
        String::from("Calendar.txt")
    );
    
    parse_args(&mut year, &mut method, &mut columns, &mut margin, &mut view, &mut mode, &mut filename);

    let calendar_text: Vec<Vec<char>> = make_calendar(view, method, year, columns, margin);

    if mode == Modes::File {
        save_to_file(filename, calendar_text);
    } else if mode == Modes::Console {
        for month in calendar_text {
            println!("{}", String::from_iter(month));
        }
    } else {
        panic!("[ERROR]: Unknown mode!")
    }
}
