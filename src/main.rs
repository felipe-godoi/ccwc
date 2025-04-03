use std::str;
use std::{
    fs,
    io::{self, BufReader, Read},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short)]
    c: bool,

    #[arg(short)]
    l: bool,

    #[arg(short)]
    w: bool,

    #[arg(short)]
    m: bool,

    filepath: Option<String>,
}

fn main() {
    let args = Args::parse();

    let filepath = &args.filepath;
    let reader = get_contents(filepath);
    let without_flags = !args.c && !args.l && !args.w && !args.m;
    let (bytes, lines, words, chars) = count_metrics(reader, &args, without_flags);

    let mut output = Vec::new();

    if args.l || without_flags {
        output.push(lines.to_string());
    }

    if args.w || without_flags {
        output.push(words.to_string());
    }

    if args.c || without_flags {
        output.push(bytes.to_string());
    }

    if args.m {
        output.push(chars.to_string());
    }

    println!(
        "{} {}",
        output.join(" "),
        filepath.to_owned().unwrap_or_default()
    );
}

fn count_metrics<R: Read>(
    mut reader: BufReader<R>,
    args: &Args,
    without_flags: bool,
) -> (usize, usize, usize, usize) {
    let mut bytes = 0;
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;

    let mut in_word = false;

    let mut buffer = [0; 8192];
    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        };

        if without_flags || args.c {
            bytes += bytes_read;
        }

        if without_flags || args.l {
            lines += buffer[..bytes_read].iter().filter(|&&b| b == b'\n').count();
        }

        if without_flags || args.w {
            for byte in &buffer[..bytes_read] {
                let is_separator = byte.is_ascii_whitespace();

                if !is_separator && !in_word {
                    in_word = true;
                    words += 1;
                } else if is_separator && in_word {
                    in_word = false;
                }
            }
        }

        if without_flags || args.m {
            chars += str::from_utf8(&buffer[..bytes_read])
                .unwrap_or("")
                .chars()
                .count();
        }
    }

    (bytes, lines, words, chars)
}

fn get_contents(filepath: &Option<String>) -> BufReader<Box<dyn Read>> {
    if let Some(filepath) = filepath {
        let file = fs::File::open(filepath).unwrap();
        BufReader::new(Box::new(file))
    } else {
        let stdin = io::stdin();
        let handle = stdin.lock();
        BufReader::new(Box::new(handle))
    }
}
