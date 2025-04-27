type Weekday = (
    Sunday    |
    Monday    |
    Tuesday   |
    Wednesday |
    Thursday  |
    Friday    |
    Saturday
);

let weekend = [
    Weekday::Sunday,
    Weekday::Saturday
];
weekend[0] != weekend[1]
