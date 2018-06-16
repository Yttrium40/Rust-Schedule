use super::Event;
use super::Schedule;
use super::Time;
use super::Day;
use super::display;
use std::fs::File;
use std::io::prelude::*;
use std::io;

pub fn export_schedule(filename: String, schedule: &Schedule) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write(&display::get_schedule_string(&schedule).into_bytes())?;
    Ok(())
}

pub fn import_schedule(filename: String) -> Result<Schedule, String> {
    unimplemented!();
}
