use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;

fn main() {
    let args = parse_args();
    println!("{:?}", args);
    let mut data = match fs::read_to_string(&args.source) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from {}. {:?}",
                "ERROR".red().bold(),
                args.source,
                e,
            );
            std::process::exit(1);
        }
    };

    data = match find_replace(&mut data, &args.input, &args.output) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace {:?}", "ERROR".red().bold(), e,);
            std::process::exit(1);
        }
    };

    match fs::write(&args.destination, data) {
        Err(e) => {
            eprintln!(
                "{} failed to write to {}. {:?}",
                "ERROR".red().bold(),
                args.source,
                e,
            );
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}

fn find_replace(text: &mut str, input: &str, output: &str) -> Result<String, regex::Error> {
    let re = Regex::new(input)?;
    Ok(re.replace_all(text, output).to_string())
}

// This attribute enables printing the
// struct directly using {:?}
#[derive(Debug)]
struct Arguments {
    source: String,
    destination: String,
    input: String,
    output: String,
}

fn print_usage() {
    eprintln!(
        "{} is basicly a find and replace tool.",
        "quickreplace".green()
    );
    eprintln!("Usage: <source> <destination> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        eprintln!(
            "{} Wrong number of arguments. Expected {}, got {}.",
            "ERROR:".red().bold(),
            4,
            args.len()
        );
        print_usage();
        std::process::exit(1);
    }

    Arguments {
        source: args[0].clone(),
        destination: args[1].clone(),
        input: args[2].clone(),
        output: args[3].clone(),
    }
}
