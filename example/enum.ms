type Weekday = (
    Sunday    |
    Monday    |
    Tuesday   |
    Wednesday |
    Thursday  |
    Friday    |
    Saturday
);

let is_weekend(the_day: Weekday) =
    the_day == Weekday::Sunday ||
    the_day == Weekday::Saturday;

let today = Weekday::Sunday;
is_weekend(today)
