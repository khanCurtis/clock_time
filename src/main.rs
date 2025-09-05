use chrono::{NaiveTime, Duration};
use std::io::{self, Write};

fn main() {
    print!("Enter clock-in time (HH:MM): ");
    io::stdout().flush().unwrap();
    let mut clock_in_str = String::new();
    io::stdin().read_line(&mut clock_in_str).unwrap();
    let clock_in_str = clock_in_str.trim();

    print!("Enter current time (HH:MM): ");
    io::stdout().flush().unwrap();
    let mut current_str = String::new();
    io::stdin().read_line(&mut current_str).unwrap();

    let current_str = current_str.trim();
    let clock_in = NaiveTime::parse_from_str(clock_in_str, "%H:%M").expect("Invalid time format");
    let current = NaiveTime::parse_from_str(current_str, "%H:%M").expect("Invalid time format");
    let shift_duration = Duration::hours(8);
    let clock_out = clock_in + shift_duration;
    let worked = current - clock_in;
    let remaining = clock_out - current;

    println!("Clock-in time: {}", clock_in.format("%H:%M"));
    println!("Current time: {}", current.format("%H:%M"));
    println!("Clock-out time: {}", clock_out.format("%H:%M"));
    println!(
        "You have worked {} hours and {} minutes.",
        worked.num_hours(),
        worked.num_minutes() % 60
    );
    println!(
            "Time remaining: {} hours and {} minutes.",
            remaining.num_hours(),
            remaining.num_minutes() % 60
        );
    }
    