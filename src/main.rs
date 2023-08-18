mod app;

use std::env;
use std::fs;
use std::io::{ErrorKind, Write};
use std::process;

use chrono::{Duration, NaiveDateTime, Utc};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use notify_rust::Notification;

const SECONDS_20_MINS: i64 = 60 * 20;

fn main() {
    let app = app::App::parse();

    match app.command {
        app::Command::Completion { shell } => {
            generate_completion(shell);
        }
        app::Command::Get { notify } => get(&notify),
        app::Command::Reset { minutes } => reset(minutes),
    }
}

fn generate_completion(shell: clap_complete::Shell) {
    let mut cmd = app::App::command();
    let cmd_name: String = cmd.get_name().into();
    generate(shell, &mut cmd, cmd_name, &mut std::io::stdout());
}

fn get(message: &str) {
    let omo_file = env::temp_dir().join(".omo");
    let contents = fs::read_to_string(&omo_file);

    match contents {
        Ok(val) => {
            let content: Vec<&str> = val.split(' ').collect();
            let stamp = content.get(0).unwrap().parse::<i64>().unwrap();
            let till_stamp = content.get(1).unwrap().parse::<i64>().unwrap();
            let diff_sec = till_stamp - stamp;
            let s = NaiveDateTime::from_timestamp(stamp, 0);
            let now = Utc::now().naive_utc();
            let duration = now.signed_duration_since(s);

            if duration.num_seconds() >= diff_sec {
                reset(Option::Some(diff_sec / 60));

                if message != "" {
                    notify(&message);
                }

                return;
            }

            let remaining = Duration::seconds(diff_sec - duration.num_seconds());

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
                reset(Option::None);
            }
            _ => {
                println!("Error reading file: {}", err);
                process::exit(1)
            }
        },
    }
}

fn reset(minutes: Option<i64>) {
    let trigger_time: i64 = match minutes {
        None => Utc::now().timestamp() + SECONDS_20_MINS,
        Some(num) => {
            Utc::now().timestamp() + (num * 60)
        }
    };

    write(Utc::now().timestamp(), trigger_time);
    get("");
}

fn print(time: String) {
    println!("\u{1F345} {}", time);
}

fn write(time: i64, seconds_delay: i64) {
    let omo_file = env::temp_dir().join(".omo");
    let mut file = match fs::File::create(omo_file) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            process::exit(1)
        }
    };

    let str_time_delay = format!("{} {}", time, seconds_delay);

    match file.write(&str_time_delay.as_bytes()) {
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
