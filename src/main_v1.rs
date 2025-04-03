use std::fs;

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

    let filepath = args.filepath;

    let contents = get_contents(&filepath);

    let mut output = String::new();

    if args.l || (!args.c && !args.l && !args.w && !args.m) {
        let lines = count_lines(&contents);
        output.push_str(format!(" {}", lines).as_str());
    }

    if args.w || (!args.c && !args.l && !args.w && !args.m) {
        let lines = count_words(&contents);
        output.push_str(format!(" {}", lines).as_str());
    }

    if args.c || (!args.c && !args.l && !args.w && !args.m) {
        let bytes = count_bytes(&contents);
        output.push_str(format!(" {}", bytes).as_str());
    }

    if args.m {
        let lines = count_chars(&contents);
        output.push_str(format!(" {}", lines).as_str());
    }

    println!("{} {}", output, filepath.unwrap_or("".to_string()));
}

fn get_contents(filepath: &Option<String>) -> String {
    if let Some(filepath) = filepath {
        fs::read_to_string(filepath).expect("Please, provide a valid filepath.")
    } else {
        let contents = std::io::stdin()
            .lines()
            .fold(String::new(), |mut acc, curr| {
                acc.push_str(curr.unwrap().as_str());
                acc.push_str("\n");
                acc
            });

        contents
    }
}

fn count_bytes(contents: &str) -> usize {
    contents.as_bytes().len()
}

fn count_lines(contents: &str) -> usize {
    contents.lines().count()
}

fn count_words(contents: &str) -> usize {
    contents.split_whitespace().count()
}

fn count_chars(contents: &str) -> usize {
    contents.chars().count()
}
