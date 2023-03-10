extern crate rust_exercises;

use rust_exercises::run;
use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(arg) => match arg.as_str() {
            "db" => run("db"),
            "bd" => run("bd"),
            _ => {
                eprintln!(
                    "\n'{}' flag does't exist! db or bd are the only options!",
                    arg
                );
                process::exit(1);
            }
        },
        None => {
            eprintln!("\nProvide arguments! db or bd");
            process::exit(1);
        }
    }
}
