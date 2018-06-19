use super::Event;
use super::Schedule;
use super::Time;
use super::Day;
use super::display;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io;

pub fn export_schedule(filename: String, schedule: &Schedule) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write(&display::get_schedule_string(&schedule).into_bytes())?;
    Ok(())
}

pub fn import_schedule(filename: String) -> Result<Schedule, Box<Error>> {
    let mut schedule = Schedule::new();
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut name = String::new();
    let mut day = Day::Sunday;
    let mut time = Time(0, 0);
    let mut location = String::new();
    let mut description = String::new();

    for line in contents.lines() {
        let mut iter = line.split_whitespace();
        if let Some(first) = iter.next() {
            if ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"].contains(&first) {
                day = Day::from_string(line.to_string())?;
                continue;
            }
            if let Some(second) = iter.next() {
                if first == "NAME:" {
                    name = second.to_string();
                    while let Some(x) = iter.next() {
                        name.push_str(" ");
                        name.push_str(x);
                    }
                } else if first == "TIME:" {
                    iter.next().unwrap();
                    let end = iter.next().unwrap();
                    time = Time::from_string((second.to_string(), end.to_string()))?;
                } else if first == "LOCATION:" {
                    location = second.to_string();
                    while let Some(x) = iter.next() {
                        location.push_str(" ");
                        location.push_str(x);
                    }
                } else if first == "DESCRIPTION:" {
                    description = second.to_string();
                    while let Some(x) = iter.next() {
                        description.push_str(" ");
                        description.push_str(x);
                    }
                    schedule.add(Event::new(&name, day, time, &location, &description))?;
                }
            }
        }
    }
    Ok(schedule)
}
