use chrono::{Duration, NaiveDateTime, Utc};
use clap::{App, Arg};
use notify_rust::Notification;
use std::env;
use std::fs;
use std::io::{ErrorKind, Write};
use std::process;

const SECONDS_20_MINS: i64 = 60 * 20;

fn main() {
    let app = App::new("omo")
        .version("1.0")
        .author("Guy Edwards <guyfedwards@gmail.com>")
        .about("Simple pomodoro timer")
        .subcommand(
            App::new("get").about("get remaining time").arg(
                Arg::with_name("notify")
                    .help("trigger system notification if 20 mins has passed when called")
                    .short("n")
                    .long("notify")
                    .value_name("MESSAGE")
                    .takes_value(true)
                    .default_value("Omo timer"),
            ),
        )
        .subcommand(App::new("reset").about("reset timer to 20 mins"))
        .get_matches();

    match app.subcommand() {
        ("get", Some(sub)) => match sub.value_of("notify") {
            Some(v) => get(v),
            None => get(""),
        },
        ("reset", Some(_)) => reset(),
        _ => {
            println!("Command must be one of [get, reset]");
            process::exit(1)
        }
    }
}

fn get(message: &str) {
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

                if message != "" {
                    notify(&message);
                }

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
    get("");
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

fn notify(message: &str) {
    match Notification::new().summary(&message).show() {
        Ok(_) => {}
        Err(e) => {
            println!("Error sending notification: {}", e);
            process::exit(1);
        }
    }
}
