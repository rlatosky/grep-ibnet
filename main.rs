use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

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

// 

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

/*
    ------------
    Program Code
    ------------
 */

struct Nodes {
    hostname: String,
    uid: String,
    portnum: i32,
    link_speed: String,
}

impl Nodes {
    pub fn build_node()
}

struct Switch {
    guid: String,
    model: String,
    // Anything plugged in - Could be switch or node
    //                uid    portnum
    devices: HashMap<String, String>,
    link_speed: String,
}

impl Switch {
    pub fn build_switch(guid: String, model: String, devices: HashMap<String, String>) -> Switch {
        Switch {
            guid: extract_guid(paragraph: &str),
            model: extract_model(paragraph: &str),
            devices: extract_devices(paragraph: &str),
            link_speed: extract_link_speeds(paragraph: &str),
        }
    }
}

fn extract_guid(paragraph: &str) {
    // get Switch IDs
    if paragraph.contains("switchguid") {
        let len_farm_hostname = 16;  // We assume farm hostname only contains six chars
        let farm_hostname_index = paragraph.find("switchguid") + 1  // Get index where 'farm' is at
        let hostname: String = paragraph.chars().skip(farm_hostname_index + 1).take(len_farm_hostname).collect();
        println!("SUBSTRING: {}", hostname);
    }
}

fn extract_model(paragraph: &str) {
    if paragraph.contains("Switch") {

        while paragraph.find("#") > paragraph.find("\"") {
            let start_index = paragraph.find("\"");
            let end_index = paragraph.find("\"").next();
        }
        let len_indexes = end_index - start_index;

        // .skip() means skipping TO that index
        let hostname: String = paragraph.chars().skip(start_index).take(len_indexes).collect();
        println!("SUBSTRING: {}", hostname);
    }
}

fn extract_nodes(paragraph: &str) {
    if paragraph.contains("Switch") {
        while paragraph.find("#") > paragraph.find("\"") {
            let start_index = paragraph.find("\"");
            let end_index = paragraph.find("\"").next();
        }
        let len_indexes = end_index - start_index;

        // .skip() means skipping TO that index
        let hostname: String = paragraph.chars().skip(start_index).take(len_indexes).collect();
        println!("SUBSTRING: {}", hostname);
    })
}

fn extract_model(paragraph: &str) {
    
}

fn extract_nodes(paragraph: &str) {
    if paragraph.contains("farm") {
        let len_farm_hostname = 6;  // We assume farm hostname only contains six chars
        let farm_hostname_index = paragraph.find("farm");   // Get index where 'farm' is at
        let farm_
        let hostname: String = paragraph.chars().skip(farm_hostname_index).take(len_farm_hostname).collect();
        paragraph.find("#");
        paragraph.find("\"");
        paragraph.find("\"").next();
        println!("SUBSTRING: {}", hostname);
    }
}
}

struct Chunks {
    nodes: vec<Nodes>,
    switches: vec<Switch>,
}

trait ChunkBuilder{
    pub fn chunkBuilder() {
        Switch.build_switch();
    }
}

fn main() {
    let args = Args::parse();
    println!("output: {:?}", cli.output);

    for paragraph in paragraphs {
        let
    }
    let paragraphs = get_paragraphs("./ibnetdiscover2023-01-02-18-29-2.txt");
    let switches = get_switches(paragraphs.clone());    // .clone() makes a copy and follows memory safety
    let nodes = get_nodes(paragraphs.clone());
    // clone args to memory safe
}

/* get_switches: read from paragraph */
fn get_switches(paragraphs: Vec<Vec<String>>) {
    let mut my_paragraph = paragraphs.clone();
}

/* get_paragraphs: start from read_lines and then save lines into a vector */
fn get_paragraphs(filename: &str) -> Vec<Vec<String>> {
    // File ibnetdiscover2023-01-02-18-29-2.txt must exist in the current path
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        Vec<Vec<String>> paragraphs;
        Vec<String> lines;
        
        for line in lines.flatten() {
            if line.trim().is_empty() {
                // New paragraph                                (Paragraph 1)        (Paragraph 2)
                paragraphs.append(lines);   // Looks like: [ [line1, line2, ...], [line1, line2, ...], ... ]
                lines.clear();
            } else {
                lines.append(line);
            }
        }
    }
    paragraphs.into();   // paragraphs is removed after this function has finished running to preserve mem
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<F>(filename: F) -> io::Result<io::Lines<io::BufReader<File>>>
where F: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
