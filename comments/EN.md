# Module:

## Presentation days

### [days.rs](../api/src/types/planets/earth/calendar/functions/days.rs)

```
The name was changed from era_days_from_date to days_from_presentation_date. In fact, we obtain from the result
of this function the count/sum of all days from the presentation date in some calendar system. However, in the
date representation itself, there may be days from another era. That's why such a name would not be entirely correct.

In the Julian calendar, these are the days 01.01.1 BCE and 02.01.1 BCE. Thus, in fact, the function returns
not exactly days from the era, but days from the representation of the date in some calendar system.
And this result SHOULD BE adjusted in the functions above to be converted to the CE (Current Era) or
BCE (Before Current Era) format.

For the Gregorian and Solar calendars, the same logical statements are also true because the correct count/sum
of days for this era is a subset of the presentation date, depending on the number of days in a year.
Even though it doesn't change anything for these calendars, the function name can still be era_days_from_date
since they do not have BCE (Before Current Era) days.

* - This functionality is NOT YET implemented.
```

### [weak.rs](../api/src/types/planets/earth/calendar/constants/week.rs)

```
The names of the constants have been changed to:

SHIFT_BEFORE_FIRST_FIRST_WEEK_DAY_SOLAR => SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_SOLAR
SHIFT_BEFORE_FIRST_FIRST_WEEK_DAY_JULIAN => SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_JULIAN
SHIFT_BEFORE_FIRST_FIRST_WEEK_DAY_DAY_GREGORIAN => SHIFT_BEFORE_FIRST_PRESENTATION_WEEK_DAY_GREGORIAN

Why? Because the presentation date from the calendar can contain days from BCE (Before Common Era).
Using the previous names would be somewhat incorrect and confusing, as such names imply that the entire
date/calendar representation belongs to the current era, leading to false assumptions.

The presentation date and the offset from the presentation date are not the same as the date in the current era.
The presentation date may include days or offsets from another era. It can be tied to days from another era to
maintain the coherence of the calendar system and its compatibility with BCE (Before Common Era). An abstract
year contains days from both the previous era and the current era.
```

### [year.rs](../api/src/types/counter/unix_time/functions/year.rs)

```
The name has been changed from year_from_era_days to year_from_presentation_days. In fact, we obtain from the result
of this function the count/sum of years from the presentation date in some calendar system. However, in the date
representation itself, there may be days from another era. That's why such a name would not be entirely correct.

In the Julian calendar, these are the days 01.01.1 BCE and 02.01.1 BCE. Thus, in fact, the function returns the year
from the representation of the date in some calendar system.

For the Gregorian and Solar calendars, the same logical statements are also true because the correct count/sum of
years for this era is a subset of the presentation date, depending on the number of days in a year. Even though it
doesn't change anything for these calendars, the function name can still be year_from_era_days since they do not
have BCE (Before Current Era) days/years.
```

## Era days

### [days.rs](../api/src/types/counter/unix_time/constants/days.rs)

```
The start day of Unix time remains unchanged because the day is independent of the date representation. The number
of days is constant and does not depend on the calendar or its formulas.

Any date is an abstraction, as is the calendar, but the day of an era is an absolute value.

According to any calendar, the day of the current era AFTER which Unix time starts counting is 719162*.
The representation of the era day* in the Julian calendar is 18.12.1969, in the Gregorian calendar is 31.12.1969,
and in the Solar calendar is 01.01.1970.

It is worth noting that the Julian representation of the date includes 2 days from BCE. 03.01.1 CE (Current Era)
in the Julian calendar corresponds to the representation of 01.01.1 in the Gregorian and Solar calendars.
If we remove the BCE days from the representation, one year in the Julian calendar would consist of 363 days,
and January would have 29 days. These BCE days (01.01.1 BCE and 02.01.1 BCE) should not be included in
the calculation of days in the current era (CE). The question is whether it is acceptable to use BCE days in
the representation of the Julian calendar.

Thus, ideally, the Julian calendar in the current era SHOULD start on Monday, 01.01.1 CE, but it currently starts
on Monday, 03.01.1 CE (including 01.01.1 BCE and 02.01.1 BCE), which is, to say the least, contradictory because
these two BCE representation days are included in the representation of the Julian date in CE.

Ideally, these days should not be present in the representation of the Julian calendar for CE, but in theory,
for the sake of backward compatibility, it may be acceptable as long as it is not confusing. The answer to why
it was done this way is not provided - it was only understood when attempting to fix the existing algorithm.
A year consisting of 363 days breaks compatibility and contradicts the "model" of a year consisting of
365/366 days and breaks all existing algorithms (except for this one, haha).

In an ideal implementation of this crate, one year for the Julian calendar should consist of 363 days, and
January should have 29 days. We can consider the days in BCE in the representation of the CE date, not for
convenience but for logical reasons. However, subsequent years would follow the same model of 365/366 days,
and the date after which Unix time starts counting would remain 18.12.1969 in the Julian representation of the date.

Practically, it is easy to implement, but it would increase the amount of code to maintain full compatibility
with BCE, continue the year counting, and maintain full compatibility between Julian representations of BCE and
CE dates without making the first year consist of 363 days and January consist of 29 days.
At this time, these changes have not yet been implemented and probably won't be.
```

## Leap Year

### [leap_year.rs](../api/src/types/planets/earth/calendar/functions/leap_year.rs)

```
- In the Julian calendar:

Every 4th year is a leap year, so we can determine a leap year unambiguously by taking the remainder of the year
divided by 4. This calendar has long lost its accuracy in terms of aligning the calendar representation and its
dates with the real seasons. For other purposes, where the date representation and the equality of seasons between
years are not important, it works perfectly. Still, due to the theory of large numbers, it will eventually return
to the starting point of seasons and date representation.

- In the Gregorian calendar:

Every 4th year is a leap year, except for years divisible by 100 (without a remainder). However, if a year is
divisible by 400 (a multiple of 400), it is a leap year (ignoring the rule for multiples of 100).
The Gregorian calendar is already ahead of the tropical year by 1 day. The theory of large numbers suggests that
using static means to determine any values in a dynamic system is futile. The Gregorian calendar will either end
with insertions or be replaced by a more accurate calendar.

- In the Solar calendar:

Also known as the Tropical calendar, it is difficult or impossible to determine a leap year unambiguously due to
the large fractional leap part - 0.24219(...). Even if it is possible, avoiding discrepancies between the calendar
representation and the seasons would be extremely challenging. Therefore, the only and most ingenious solution is
to approach it in reverse (for which insight was needed).

For example, during the first 4 years in the Solar/Tropical calendar, the following occurs:

4 * 0.24219 = 0.96876 (leap days)

This is why the 4th year cannot be a leap year a priori, but the next one can be.

How does the formula work and why does it work? The formula works as follows:

1) Take the current year, for example, the 5th (year).
2) Take the previous year, the 4th.
3) Calculate the fractional leap value for the previous year, 0.96876.
4) Add the fractional leap part (0.24219) from the next year (5th) to 0.96876, resulting in 1.21095 (at the end of the 5th year).
5) Compare the integer parts:
5.1) When the integer leap part of the next year (1) is greater than that of the previous year (0): 1 > 0, the year is a leap year.

Thus, the accuracy of the calendar system now depends only on the constant leap part of the year, which is determined
as 0.24219(..) for the Solar calendar. This is beneficial as it eliminates situations where insertions are made
while simultaneously increasing the accuracy of the calendar system.
```

## Timezone

### [tz.rs](../api/src/types/planets/earth/calendar/traits/converter/tz.rs)

#### Signed Timezone:

```
1. If the time zone is negative and the number of seconds in the last day (in UTC - Unix time) is greater than or
equal to the absolute value of the time zone, subtract the number of seconds in the time zone and add the number of
seconds for the current day. The number of days remains unchanged! era_days: [0, 0].

2. If the time zone is negative and contains more seconds than the number of seconds in the last day (day_seconds)
(i.e., it is always from [-1, -N] days in terms of seconds in UTC - Unix time), calculate the difference between the
time zone and the seconds in the current day. This difference represents the amount of time that the current day
deviates backward in Unix time from the time zone. The minimum value of this difference is 1 second (23:59:59) or
1 day ago. The division with rounding up is a trick to obtain the absolute sum of days that the current day deviates
backward. This expression is always within the range of [-1, -N] days relative to the era days/seconds in UTC,
regardless of the time zone! era_days: [-1, -N]. After that, subtract the number of seconds in the time zone and
add the number of seconds for the current day.
```

#### Unsigned Timezone:

```
1. If the time zone is positive (relative to UTC) and the total sum of the time zone seconds (tz_sec) and the seconds
in the last day (day_seconds) is strictly less than or does not exceed the maximum number of seconds required for
a day change (SECONDS_IN_DAY), the day remains the same, and the number of days remains unchanged! era_days: [0, 0].

2. If the time zone is positive (relative to UTC) and the total sum of the time zone seconds (tz_sec) and the seconds
in the last day (day_seconds) is greater than or equal to the minimum value required for a day change (SECONDS_IN_DAY),
perform Euclidean division (integer division with rounding down) of the sum of the time zone seconds (tz_sec) and the
seconds in the last day (day_seconds) by SECONDS_IN_DAY to determine the sum of excess days (always from [+1, +N] days
in terms of seconds in UTC - Unix time). Increase the number of era days by this value. This expression is always
within the range of [+1, +N] days relative to the era days/seconds in UTC, regardless of the time zone!
era_days: [+1, +N]. After that, add the number of seconds in the time zone and the number of seconds for the
current day.
```