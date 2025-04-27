type Weekday = (
    Sunday    |
    Monday    |
    Tuesday   |
    Wednesday |
    Thursday  |
    Friday    |
    Saturday
);

let is_weekend(the_day: Weekday) = {
    (Weekday::Sunday   == the_day) ||
    (Weekday::Saturday == the_day)
};

let today = Weekday::Sunday;
is_weekend(today)
