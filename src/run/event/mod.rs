pub mod schedule;

pub struct Time(u16, u16);

pub struct Event {
    pub name: String,
    pub time: Time,         // uses 24-hr time internally, e.g. 1230 for 12:30 p.m.
    pub location: String,
    pub description: String,
}

impl Event {
    fn new(name: &str, time: Time, location: &str, description: &str) -> Event {
        Event {
            name: String::from(name),
            time,
            location: String::from(location),
            description: String::from(description),
        }
    }
}
