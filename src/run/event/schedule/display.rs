use super::Event;
use super::Schedule;
use super::Time;
use super::Day;
use std::cmp::Ordering;
use std::mem::discriminant;

pub fn display_event(id: u8, schedule: &Schedule) -> Result<(), String> {
    if let Some(event) = schedule.get(id) {
        println!("{}", get_event_string(&event));
        Ok(())
    } else {
        Err(String::from("ID not found."))
    }
}

pub fn get_event_string(event: &Event) -> String {
    let time = Time::to_string(&event.time);
    format!("NAME:        {name}
DAY:         {day}
TIME:        {start} - {end}
LOCATION:    {location}
DESCRIPTION: {description}",
             name = event.name,
             day = Day::to_string(&event.day),
             start = time.0,
             end = time.1,
             location = event.location,
             description = event.description)
}

pub fn display_schedule(schedule: &Schedule) {
    let string = get_schedule_string(&schedule);
    println!("{}", string);
}

pub fn get_schedule_string(schedule: &Schedule) -> String {
    let mut string: String = "".to_string();
    for day in [Day::Sunday, Day::Monday, Day::Tuesday, Day::Wednesday, Day::Thursday, Day::Friday, Day::Saturday].iter() {
        let mut events: Vec<&Event> = vec!();
        for event in schedule.table.iter() {
            if discriminant(&event.1.day) == discriminant(day) {
                events.push(&event.1);
            }
        }
        if !events.is_empty() {
            events.sort_unstable_by(|e1, e2|
                match e1.time.0 < e2.time.0 {
                    true => Ordering::Less,
                    false => Ordering::Greater,
                }
            );
            string.push('\n');
            string.push_str(&Day::to_string(&day));
            string.push('\n');
            string.push('\n');
            for event in events {
                string.push_str(&get_event_string(&event));
                string.push('\n');
                string.push('\n');
            }
        }
    }
    String::from(string)
}
