use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

use structures::{Node, Switch, Chunk};

// Clap used for arguments
use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;

/*
    Some Information:
    ->
    https://stackoverflow.com/questions/43820696/how-can-i-find-the-index-of-a-character-in-a-string-in-rust
 */
// clap library for argument management
// take from std::in

use clap::Parser;

#[derive(Parser)]
#[command(name = "grep-ibnet")]
#[command(author = "Rylen L. <latosky@jlab.org>")]
#[command(version = "0.01")]
#[command(about = "Format the infini-band status file and return into JSON", long_about = None)]
struct Args {
    #[arg(long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    println!("output: {:?}", cli.output);

    let file = get_paragraphs("./ibnetdiscover2023-01-02-18-29-2.txt");

    for paragraph in file.clone() {
        let Switch = build_switch(&paragraph);
        let Node = build_node(&paragraph);
        let Chunk = build_chunk(&paragraph);
    }
}

/* get_switches: read from paragraph */
fn get_switches(paragraph: Vec<String>) {
    let mut my_paragraph = paragraph.clone();
    for line in paragraph {
        
    }
}

/* get_paragraphs: start from read_lines and then save lines into a vector */
fn get_paragraphs(filename: &str) -> Vec<Vec<String>> {
    // File ibnetdiscover2023-01-02-18-29-2.txt must exist in the current path
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        Vec<Vec<String>> file;
        Vec<String> templines;
        
        for line in lines.flatten() {
            if line.trim().is_empty() {
                // New paragraph                                (Paragraph 1)        (Paragraph 2)
                file.push(templines);   // File looks like: [ [line1, line2, ...], [line1, line2, ...], ... ]
                templines.clear();
            } else {
                templines.push(line);
            }
        }
    }
    file.into();    // paragraph is removed after this function has finished running to preserve mem
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<F>(filename: F) -> io::Result<io::Lines<io::BufReader<File>>>
where F: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
