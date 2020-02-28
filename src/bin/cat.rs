use std::{env, fs, io, process};

fn usage(msg: Option<&str>) {
    match msg {
        Some(s) => println!("error: {}", s),
        None => (),
    }
    println!("usage: cat [files ...]");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args.len();

    if n == 1 {
        return match io::copy(&mut io::stdin(), &mut io::stdout()) {
            Ok(_) => (),
            Err(_) => usage(Some("unable to read <stdin>")),
        };
    }

    for arg in args[1..n].iter() {
        let mut f = match fs::File::open(arg) {
            Ok(f) => f,
            Err(_) => return usage(Some("unable to open file")),
        };

        match io::copy(&mut f, &mut io::stdout()) {
            Ok(_) => (),
            Err(_) => usage(Some("unable to read file")),
        };
    }
}
