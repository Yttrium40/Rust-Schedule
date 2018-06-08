use super::Event;
use super::Schedule;
use super::Time;
use super::Day;
use std::cmp::Ordering;
use std::mem::discriminant;

pub fn display_event(id: u8, schedule: &Schedule) -> Result<(), String> {
    if let Some(event) = schedule.get(id) {
        println!("NAME:        {}", event.name);
        println!("DAY:         {}", Day::to_string(&event.day));
        let time = Time::to_string(&event.time);
        println!("TIME:        {} - {}", time.0, time.1);
        println!("LOCATION:    {}", event.location);
        println!("DESCRIPTION: {}", event.description);

        Ok(())
    } else {
        Err(String::from("ID not found."))
    }
}

pub fn display_schedule(schedule: &Schedule) -> Result<(), String> {
    // let sorting = |e1: &Event, e2: &Event| {
    //     match e1.time.0 < e2.time.0 {
    //         true => Ordering::Less,
    //         false => Ordering::Greater,
    //     }
    // };
    for day in [Day::Sunday, Day::Monday, Day::Tuesday, Day::Wednesday, Day::Thursday, Day::Friday, Day::Saturday].iter() {
        let mut events: Vec<&Event> = vec!();
        for event in schedule.table.iter() {
            if discriminant(&event.1.day) == discriminant(day) {
                events.push(&event.1);
            }
        }
        events.sort_unstable_by(|e1, e2|
            match e1.time.0 < e2.time.0 {
                true => Ordering::Less,
                false => Ordering::Greater,
            }
        );
        if !events.is_empty() {
            println!("{}", Day::to_string(&day));
            for event in events {
                display_event(event.id, &schedule)?;
            }
        }
    }
    Ok(())
}
