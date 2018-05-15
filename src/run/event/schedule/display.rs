use super::Event;
use super::Schedule;
use super::Time;
use super::Day;

pub fn display_event(id: u8, schedule: &Schedule) -> Result<(), String> {
    let event = schedule.get(id)?;
    println!("NAME:        {}", event.name);
    println!("DAY:         {}", Day::to_string(&event.day));
    println!("TIME:        {} - {}", event.time.0, event.time.1);
    println!("LOCATION:    {}", event.location);
    println!("DESCRIPTION: {}", event.description);

    Ok(())
}

pub fn display_schedule(schedule: &Schedule) -> Result<(), String> {
    unimplemented!();
}
