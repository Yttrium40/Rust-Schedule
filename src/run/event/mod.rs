pub mod schedule;

pub struct Time(pub u16,pub u16);

impl Time {
    pub fn to_string(time: &Time) -> String {
        unimplemented!(); // TODO
    }
}

pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Day {
    pub fn to_string(day: &Day) -> String {
        let s = match day {
            Day::Sunday => "Sunday",
            Day::Monday => "Monday",
            Day::Tuesday => "Tuesday",
            Day::Wednesday => "Wednesday",
            Day::Thursday => "Thursday",
            Day::Friday => "Friday",
            Day::Saturday => "Saturday",
        };
        String::from(s)
    }
}

pub struct Event {
    pub name: String,
    pub day: Day,
    pub time: Time,         // uses 24-hr time internally, e.g. 1230 for 12:30 p.m.
    pub location: String,
    pub description: String,
}

impl Event {
    pub fn new(name: &str, day: Day, time: Time, location: &str, description: &str) -> Event {
        Event {
            name: String::from(name),
            day,
            time,
            location: String::from(location),
            description: String::from(description),
        }
    }
}
