extern crate test_here;

use std::env;
use std::process;
use test_here::run;

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
            eprintln!("\nProvide arguements! db or bd");
            process::exit(1);
        }
    };
}
