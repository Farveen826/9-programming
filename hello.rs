use std::process::Command;

fn main() {
    let output = Command::new("date")
        .arg("+%Y-%m-%d %H:%M:%S")
        .output()
        .expect("Failed to get time");

    let time = String::from_utf8_lossy(&output.stdout);

    println!("Hello Farveen, right now the time is {}", time.trim());
}
