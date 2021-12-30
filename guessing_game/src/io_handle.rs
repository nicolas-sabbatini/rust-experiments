use std::io::{self, Write};
use std::{thread, time};

/// Read line from standard input.
pub fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line.");
    line.trim().to_string()
}

/// Prints text to standard output.
pub fn flush_text(txt: &str) {
    print!("{}", txt);
    io::stdout().flush().expect("Failed to flush stdout.");
}

/// Prints to the standard output, with a newline.
pub fn flush_line(txt: &str) {
    println!("{}", txt);
    io::stdout().flush().expect("Failed to flush stdout.");
}

/// Prints to the standard output in a timed manner.
pub fn timed_flush_text(txt: &str, sec: f32) {
    let sleep_time = time::Duration::from_secs_f32(sec / txt.len() as f32);
    for c in txt.chars() {
        print!("{}", c);
        io::stdout().flush().expect("Failed to flush stdout.");
        thread::sleep(sleep_time);
    }
}

