use std::process::Command;

fn main() {
    // Get the current date and time from the system
    let output = Command::new("date")
        .arg("+%Y-%m-%d %H:%M:%S")
        .output()
        .expect("Failed to execute date command");

    // Parse output into date and time strings
    let datetime_str = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    let (date, time) = datetime_str.trim().split_once(' ').unwrap();

    // Pass date and time as environment variables
    println!("cargo:rustc-env=BUILD_DATE={}", date);
    println!("cargo:rustc-env=BUILD_TIME={}", time);
}