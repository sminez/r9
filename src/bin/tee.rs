use clap::Clap;
use std::io::*;
use std::{fs, io, process};

/// tee - send the output of a command to one or more files in addition to stdout
#[derive(Clap)]
struct Options {
    /// append to files rather than overwriting them
    #[clap(short = "a", long = "append")]
    append: bool,

    /// the files to tee stdin into
    files: Vec<String>,
}

fn open_files(paths: Vec<String>, append: bool) -> Vec<fs::File> {
    return paths
        .iter()
        .flat_map(|path| {
            if append && fs::metadata(&path).is_ok() {
                fs::OpenOptions::new().append(true).open(&path)
            } else {
                fs::File::create(&path)
            }
        })
        .collect();
}

fn main() {
    let opts: Options = Options::parse();
    let mut files = open_files(opts.files, opts.append);

    for res in io::stdin().lock().lines() {
        match res {
            Ok(mut line) => {
                line.push('\n');
                for f in &mut files {
                    match f.write(line.as_bytes()) {
                        Err(e) => {
                            println!("write error: {}", e);
                            process::exit(1);
                        }
                        _ => (),
                    }
                }
                print!("{}", line); // send to stdout as well as the files
            }
            Err(e) => {
                println!("error reading from <stdin>: {}", e);
                process::exit(1);
            }
        }
    }
}
