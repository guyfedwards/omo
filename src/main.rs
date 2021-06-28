use chrono::{Duration, NaiveDateTime, Utc};
use notify_rust::Notification;
use std::env;
use std::fs;
use std::io::{ErrorKind, Write};
use std::process;

enum Cmd {
    Get,
    Reset,
}

const SECONDS_20_MINS: i64 = 60 * 20;

impl Cmd {
    fn from_string(input: &str) -> Option<Cmd> {
        match input {
            "get" => Some(Cmd::Get),
            "reset" => Some(Cmd::Reset),
            _ => None,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Must provide a command");
        process::exit(1)
    }

    let cmd = Cmd::from_string(&args[1]);

    match cmd {
        Some(Cmd::Get) => get(),
        Some(Cmd::Reset) => {
            reset();
        }
        None => {
            println!("Command must be one of [get, reset]");
            process::exit(1)
        }
    }
}

fn get() {
    let omo_file = env::temp_dir().join(".omo");
    let contents = fs::read_to_string(&omo_file);

    match contents {
        Ok(val) => {
            let stamp = val.parse::<i64>().unwrap();
            let s = NaiveDateTime::from_timestamp(stamp, 0);
            let now = Utc::now().naive_utc();
            let duration = now.signed_duration_since(s);

            if duration.num_minutes() >= 20 {
                reset();
                notify();
                return;
            }

            let remaining = Duration::seconds(SECONDS_20_MINS - duration.num_seconds());

            print(format!(
                "{:0>2}:{:0>2}",
                remaining.num_minutes(),
                remaining.num_seconds() % 60,
            ));
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                fs::File::create(&omo_file).unwrap_or_else(|err| {
                    println!("Error creating file: {}", err);
                    process::exit(1);
                });
                reset();
            }
            _ => {
                println!("Error reading file: {}", err);
                process::exit(1)
            }
        },
    }
}

fn reset() {
    write(Utc::now().timestamp());
    print(String::from("20:00"));
}

fn print(time: String) {
    println!("\u{1F345} {}", time);
}

fn write(time: i64) {
    let str_time = time.to_string();
    let omo_file = env::temp_dir().join(".omo");
    let mut file = match fs::File::create(omo_file) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            process::exit(1)
        }
    };

    match file.write(&str_time.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            println!("Error writing file: {}", e);
            process::exit(1)
        }
    }
}

fn notify() {
    match Notification::new().summary("Omo alert").show() {
        Ok(_) => {}
        Err(e) => {
            println!("Error sending notification: {}", e);
            process::exit(1);
        }
    }
}
