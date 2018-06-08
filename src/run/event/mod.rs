pub mod schedule;

pub struct Time(pub u16, pub u16);

impl Time {
    // string format for times is hr:min
    pub fn from_string((str1, str2): (String, String)) -> Result<Time, String> {
        let conversion_closure = |acc: &u16, (i, c): (u32, &char)| -> Result<u16, String> {
            if let Some(y) = c.to_digit(10) {
                let num = y as u16;
                Ok(acc + num * 10u16.pow(i))
            } else {
                Err(String::from("Could not convert string to time"))
            }
        };

        let mut vec: Vec<&str> = str1.as_str().rsplit("").collect();
        let mut vec_char: Vec<char> = vec!();
        for c in vec {
            if c != "" {
                vec_char.push(c.chars().next().unwrap());
            }
        }
        match vec_char[2] {
            ':' => vec_char.remove(2),
            _ => return Err(String::from("Invalid time format")),
        };
        let mut start: u16 = 0;
        for (i, c) in vec_char.iter().enumerate() {
            start = conversion_closure(&start, (i as u32, &c))?;
        }

        vec = str2.as_str().rsplit("").collect();
        vec_char = vec!();
        for c in vec {
            if c != "" {
                vec_char.push(c.chars().next().unwrap());
            }
        }
        match vec_char[2] {
            ':' => vec_char.remove(2),
            _ => return Err(String::from("Invalid time format")),
        };
        let mut end: u16 = 0;
        for (i, c) in vec_char.iter().enumerate() {
            end = conversion_closure(&end, (i as u32, &c))?;
        }

        Ok(Time(start, end))
    }

    pub fn to_string(time: &Time) -> (String, String) {
        let mut first = time.0.to_string();
        let mut index = first.len() - 2;
        first.insert(index, ':');
        let mut second = time.1.to_string();
        index = second.len() - 2;
        second.insert(index, ':');
        (first, second)
    }
}

#[derive(PartialEq)]
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
    pub fn from_string(string: String) -> Result<Day, String> {
        match string.to_lowercase().as_str() {
            "sunday" => Ok(Day::Sunday),
            "monday" => Ok(Day::Monday),
            "tuesday" => Ok(Day::Tuesday),
            "wednesday" => Ok(Day::Wednesday),
            "thursday" => Ok(Day::Thursday),
            "friday" => Ok(Day::Friday),
            "saturday" => Ok(Day::Saturday),
            _ => Err(String::from("Invalid day.")),
        }
    }

    pub fn to_string(day: &Day) -> String {
        let s = match day {
            &Day::Sunday => "Sunday",
            &Day::Monday => "Monday",
            &Day::Tuesday => "Tuesday",
            &Day::Wednesday => "Wednesday",
            &Day::Thursday => "Thursday",
            &Day::Friday => "Friday",
            &Day::Saturday => "Saturday",
        };
        String::from(s)
    }
}

pub struct Event {
    pub id: u8,
    pub name: String,
    pub day: Day,
    pub time: Time,         // uses 24-hr time internally, e.g. 1230 for 12:30 p.m.
    pub location: String,
    pub description: String,
}

impl Event {
    pub fn new(name: &str, day: Day, time: Time, location: &str, description: &str) -> Event {
        Event {
            id: 100,
            name: String::from(name),
            day,
            time,
            location: String::from(location),
            description: String::from(description),
        }
    }
}
