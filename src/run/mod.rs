mod event;

use std::io;
use std::error::Error;
use self::event::Event;
use self::event::Time;
use self::event::schedule::Schedule;

struct Options {
    twelve_hour: bool,
}

impl Options {
    fn new(twelve_hour: bool) -> Options {
        Options {
            twelve_hour,
        }
    }

    fn reset(&mut self) {
        self.twelve_hour = false;
    }
}

pub fn run() -> Result<(), Box<Error>> {
    println!("Enter 1 to start a new schedule, or 2 to import a text file: ");
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
    let schedule = Schedule::new();
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
        println!("8: Exit");

        input = read_input()?;

        match input.as_str() {
            "1" => (),
            "2" => (),
            "3" => (),
            "4" => (),
            "5" => (),
            "6" => (),
            "7" => (),
            "8" => (),
            _ => println!("Enter a command."),
        }
    }
}

fn read_input() -> Result<String, Box<Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(String::from(input.trim()))
}
