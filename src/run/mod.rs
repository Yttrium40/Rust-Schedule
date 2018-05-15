mod event;

use std::io;
use std::io::Write;
use std::error::Error;
use std::process;
use run::event::Event;
use run::event::Time;
use run::event::Day;
use run::event::schedule::Schedule;
use run::event::schedule::display;

pub struct Settings {
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

static mut SETTINGS: Settings = Settings {
        twelve_hour: false,
    };

pub fn run() -> Result<(), Box<Error>> {
    loop {
        print("Enter 1 to start a new schedule, or 2 to import a text file: ");
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
                _ => print("Enter either 1 or 2: "),
            };
        }
        let mut schedule = Schedule::new();
        if input == "2" {
            println!("Import functionality coming soon!");
        }

        loop {
            println!("----------");
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
            println!("----------");
            print("Enter a command: ");

            input = read_input()?;

            let result = match input.as_str() {
                "1" => run_display_schedule(&schedule),
                "2" => run_display_event(&schedule),
                "3" => run_add(&mut schedule),
                "4" => run_remove(&mut schedule),
                "5" => run_edit(&mut schedule),
                "6" => run_export(&schedule),
                "7" => break,
                "8" => run_settings(),
                "9" => process::exit(0),
                _ => {
                    print("Enter a command: ");
                    Ok(())
                },
            };
            if let Err(e) = result {
                println!("ERROR IN OPERATION: {}", e);
                print("Press enter to continue");
                read_input()?;
            }
        }
    }
}

fn run_display_schedule(schedule: &Schedule) -> Result<(), Box<Error>> {
    unimplemented!();
}

fn run_display_event(schedule: &Schedule) -> Result<(), Box<Error>> {
    print("Enter an event ID: ");
    let id = read_input_id();
    println!();
    display::display_event(id, &schedule)?;
    println!();
    print("Press enter to continue");
    read_input()?;

    Ok(())
}

fn run_add(schedule: &mut Schedule) -> Result<(), Box<Error>> {
    print("Enter an event name: ");
    let name = read_input()?;

    print("Enter a day: ");
    let mut day = Day::Sunday;
    loop {
        let input = read_input()?;
        day = match input.to_lowercase().as_str() {
            "sunday" => Day::Sunday,
            "monday" => Day::Monday,
            "tuesday" => Day::Tuesday,
            "wednesday" => Day::Wednesday,
            "thursday" => Day::Thursday,
            "friday" => Day::Friday,
            "saturday" => Day::Saturday,
            _ => {
                print("Enter a day: ");
                continue;
            }
        };
        break;
    }

    print("Enter a starting time: ");
    let mut start = 0;
    loop {
        match read_input().unwrap().parse::<u16>() {
            Ok(i) => {
                start = i;
                break;
            },
            Err(_) => print("Enter a time: "),
        };
    }
    print("Enter an ending time: ");
    let mut end = 0;
    loop {
        match read_input().unwrap().parse::<u16>() {
            Ok(i) => {
                end = i;
                break;
            },
            Err(_) => print("Enter a time: "),
        };
    }
    let time = Time(start, end);

    print("Enter a location: ");
    let location = read_input()?;

    print("Enter a description: ");
    let description = read_input()?;

    let id = schedule.add(Event::new(&name, day, time, &location, &description))?;
    println!("New event created with ID {}", id);

    Ok(())
}

fn run_remove(schedule: &mut Schedule) -> Result<(), Box<Error>> {
    print("Enter an event ID: ");
    let id = read_input_id();
    schedule.remove(id)?;
    println!("Event removed.");
    Ok(())
}

fn run_edit(schedule: &mut Schedule) -> Result<(), Box<Error>> {
    unimplemented!();
}

fn run_export(schedule: &Schedule) -> Result<(), Box<Error>> {
    unimplemented!();
}

fn run_settings() -> Result<(), Box<Error>> {
    unimplemented!();
}

fn print(string: &str) {
    print!("{}", string);
    io::stdout().flush().unwrap();
}

fn read_input() -> Result<String, Box<Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(String::from(input.trim()))
}

fn read_input_id() -> u8 {
    let mut id = 0;
    loop {
        match read_input().unwrap().parse::<u8>() {
            Ok(i) => if i < 100 {
                id = i;
                break;
            },
            Err(_) => print("Enter a number less than 100: "),
        };
    }
    id
}
