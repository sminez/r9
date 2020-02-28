use std::{env, process, thread, time};

fn usage(msg: Option<&str>) {
    match msg {
        Some(s) => println!("error: {}", s),
        None => (),
    }
    println!("usage: sleep [duration]");
    process::exit(1);
}

fn main() {
    match env::args().nth(1) {
        Some(s) => match s.parse::<u64>() {
            Ok(n) => thread::sleep(time::Duration::from_secs(n)),
            Err(_) => usage(Some("duration must be an integer")),
        },
        None => usage(None),
    };
}
