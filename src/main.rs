use colored::Colorize;
// use std::io::prelude::*;
// use std::io::BufReader;
use std::fs::File;
use clap::Parser;
use rev_lines::RevLines;
use std::string::String;

#[derive(Parser, Debug)]
#[command(name = "hedr")]
#[command(about = "Print the first 10 lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n\nWith no FILE, or when FILE is -, read standard input.")]
#[command(long_about = None)]
#[command(version = ".1")]
struct Args {
    fnames: Vec<String>,

    #[arg(short, long, default_value_t = 10)]
    number_of_lines: u32,

    #[arg(short, long, default_value_t = 0)]
    character_count: u32,
}

fn main() {
    let args = Args::parse();

    if args.fnames.len() > 1 {
        for fname in args.fnames {
            println!("==> {} <==", fname.bold().green());
            output_tail_of_file(args.character_count, args.number_of_lines, &fname);
            println!();
        }
    } else {
        output_tail_of_file(args.character_count, args.number_of_lines, &args.fnames[0]);
    }
}

fn output_tail_of_file(character_count: u32, number_of_lines: u32, fname: &str) {
    let reader = RevLines::new(File::open(fname).expect("Opening fname failed"));

    let mut character_out = String::from("");

    if character_count > 0 {
        for (iterator, line) in reader.enumerate() {
            for character in line.expect("Reading lines failed").chars() {
                if iterator >= character_count.try_into().unwrap() {
                    break;
                }
                character_out.insert_str(0, &(character.to_string()));
                // iterator+=1;
            }
            println!(); // adds newline back in - feels hacky, will try and fix later

            if iterator >= character_count.try_into().unwrap() {
                break;
            }
        }
    } else {
        let mut string_out = String::from("");
        // let mut iterator = 0;
        for (iterator, line) in reader.enumerate() {
            if iterator >= number_of_lines.try_into().unwrap() {
                break;
            }
            let mut newl = line.expect("Reading lines failed");
            newl.push('\n');
            string_out.insert_str(0, &newl);
            // iterator+=1;
        }
        println!("{}", string_out)
    }
}
