use env_logger::{Builder, Target};
use log::*;
use std::env;
use std::io;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), io::Error> {
    let mut builder = Builder::new();
    builder
        .format(|buf, record| writeln!(buf, "CLIENT {} - {}", record.level(), record.args()))
        .target(Target::Stderr)
        .filter(None, LevelFilter::Debug)
        .init();
    let args: Vec<String> = env::args().collect();
    debug!("process_client args: {:?}", args);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut line = String::new();

    let mut count = 0usize;
    loop {
        let n = stdin.read_line(&mut line).expect("read_to_string failed");

        debug!("[{}] received line: \"{}\"", count, line);
        if line == "exit" {
            debug!("exiting!");
            break;
        }
        if n == 0 {
            debug!("stdin input had len 0...");
            if count >= 10 {
                debug!("exiting after {} retries", count);
                break;
            }
        }

        // Write the line to stdout.
        stdout
            .write_all(b"response\n")
            .expect("writing to stdout failed");
        stdout.flush().unwrap();
        line.clear();
        count += 1;
        thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}
