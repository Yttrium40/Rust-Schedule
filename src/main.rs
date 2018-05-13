mod run;

use std::process;

fn main() {
    if let Err(e) = run::run() {
        eprintln!("{}", e);

        process::exit(1);
    }
}
