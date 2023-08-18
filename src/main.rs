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
        app::Command::Reset => reset(),
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
