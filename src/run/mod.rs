mod event;

use std::io;
use std::error::Error;
use std::process;
use run::event::Event;
use run::event::Time;
use run::event::schedule::Schedule;

struct Settings {
    twelve_hour: bool,
}

impl Settings {
    fn new() -> Settings {
        Settings {
            twelve_hour: false,
        }
    }

    fn reset(&mut self) {
        self.twelve_hour = false;
    }
}

pub fn run() -> Result<(), Box<Error>> {
    loop {
        println!("Enter 1 to start a new schedule, or 2 to import a text file.");
        let mut input = String::new();

        loop {
            input = read_input()?;
            match input.as_str() {
                "1" => {
                    println!("New schedule!");
                    break;
                },
                "2" => {
                    println!("Import schedule!");
                    break;
                },
                _ => println!("Enter either 1 or 2."),
            };
        }
        let mut schedule = Schedule::new();
        let mut settings = Settings::new();
        if input == "2" {
            println!("Import functionality coming soon!");
        }

        loop {
            println!("Commands:");
            println!("1: Display schedule");
            println!("2: Display specific event");
            println!("3: Add event");
            println!("4: Remove event");
            println!("5: Edit event");
            println!("6: Export schedule");
            println!("7: Return to new/import schedule");
            println!("8: Change settings");
            println!("9: Exit");

            input = read_input()?;

            match input.as_str() {
                "1" => run_display_schedule(&schedule)?,
                "2" => run_display_event(&schedule)?,
                "3" => run_add(&mut schedule)?,
                "4" => run_remove(&mut schedule)?,
                "5" => run_edit(&mut schedule)?,
                "6" => run_export(&schedule)?,
                "7" => break,
                "8" => run_settings(&mut settings)?,
                "9" => process::exit(0),
                _ => println!("Enter a command."),
            };
        }
    }
}

fn run_display_schedule(schedule: &Schedule) -> Result<(), Box<Error>> {
    Ok(())
}

fn run_display_event(schedule: &Schedule) -> Result<(), Box<Error>> {
    Ok(())
}

fn run_add(schedule: &mut Schedule) -> Result<(), Box<Error>> {
    println!("Enter an event name");
    let name = read_input()?;

    println!("Enter a starting time");
    let mut start = 0;
    loop {
        match read_input().unwrap().parse::<u16>() {
            Ok(i) => {
                start = i;
                break;
            },
            Err(_) => println!("Enter a time"),
        };
    }
    println!("Enter an ending time");
    let mut end = 0;
    loop {
        match read_input().unwrap().parse::<u16>() {
            Ok(i) => {
                end = i;
                break;
            },
            Err(_) => println!("Enter a time"),
        };
    }
    let time = Time(start, end);

    println!("Enter a location");
    let location = read_input()?;

    println!("Enter a description");
    let description = read_input()?;

    schedule.add(Event::new(&name, time, &location, &description))?;

    Ok(())
}

fn run_remove(schedule: &mut Schedule) -> Result<(), Box<Error>> {
    Ok(())
}

fn run_edit(schedule: &mut Schedule) -> Result<(), Box<Error>> {
    Ok(())
}

fn run_export(schedule: &Schedule) -> Result<(), Box<Error>> {
    Ok(())
}

fn run_settings(settings: &mut Settings) -> Result<(), Box<Error>> {
    Ok(())
}

fn read_input() -> Result<String, Box<Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(String::from(input.trim()))
}
