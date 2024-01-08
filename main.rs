use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use substring::Substring;

// Clap used for arguments
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

// Nodes include [H] AND [S] - anything below a Switch
struct Node {
    hostname: String,
    uid: String,
    portnum: String,
    link_speed: String,
}

impl Node {
    pub fn build_node(hostname: String, uid: String, portnum: String, link_speed: String) -> Node {
        return Node {
            hostname: hostname,
            uid: uid,
            portnum: portnum,
            link_speed: link_speed,
        }
    }
}

struct Switch {
    guid: String,
    model: String,
    devices: Vec<Node>,
    link_speeds: Vec<String>,
}

impl Switch {
    pub fn build_switch(paragraph: &Vec<String>) -> Switch {
        return Switch {
            guid: extract_switch_guid(paragraph),
            model: extract_switch_model(paragraph),
            devices: extract_switch_devices(paragraph),
            link_speeds: extract_switch_link_speeds(paragraph),
        }
    }

}

fn extract_switch_guid(paragraph: &Vec<String>) -> String {
    let my_paragraph = paragraph.clone();
    let mut guid = "<None>";
    for line in my_paragraph {
        if line.contains("switchguid") {
            let start_index = line.find("(").unwrap_or(line.len());
            let end_index = line.find(")").unwrap_or(line.len());
            guid = line.substring(start_index, end_index);
            println!("GUID SUBSTRING: {}", guid);
            return guid.into();
        } else {
            return guid.into();
        }
    }
    return guid.to_string();
}

fn extract_switch_model(paragraph: &Vec<String>) -> String {
    let my_paragraph = paragraph.clone();
    let mut model = "<None>";
    for line in my_paragraph {
        if line.contains("Switch") {
            let mut start_index = 0;
            let mut end_index = 0;
            while line.find("#") > line.find("\"") {
                start_index = line.find("\"").unwrap_or(line.len());
                end_index = line.rfind("\"").unwrap_or(line.len());
            }
            model = line.clone().substring(start_index, end_index);
            println!("MODEL SUBSTRING: {}", model);
        }
    }
    return model.to_string();
}
// format as a HashMap(uid, portnum)
// collect devices as a hashmap from '[]'
fn extract_switch_devices(paragraph: &Vec<String>) -> Vec<Node> {
    let mut my_paragraph = paragraph.clone();
    let mut found_uid;
    let mut found_portnum;
    let mut found_hostname;
    let mut found_link_speeds;
    let mut found_nodes: Vec<Node> = Vec::new();

    for line in my_paragraph { // working with String
        let mut start_index: usize;
        let mut end_index: usize;
        let device_code: String = line.substring(line.find("\"").unwrap_or(line.len()),line.find("\"").unwrap_or(line.len())+1).to_string();
        let last_word = line.split(' ').last().unwrap();
        if line.contains("[") {
            start_index = line.find("(").unwrap_or(line.len());
            end_index = line.find(")").unwrap_or(line.len());
            found_uid = line.substring(start_index, end_index);

            start_index = line.find("[").unwrap_or(line.len());
            end_index = line.find("]").unwrap_or(line.len());

            found_portnum = line.trim().substring(start_index, end_index);
            while line.contains("#") < line.contains("\"") {
                start_index = line.find("\"").unwrap_or(line.len());
                end_index = line.rfind("\"").unwrap_or(line.len());
            }
            found_hostname = line.substring(start_index, end_index);
            found_link_speeds = last_word.to_string();

            found_nodes.push(Node::build_node(found_hostname.to_string(), found_portnum.to_string(),
                found_uid.to_string(), found_link_speeds.to_string()));
        }
    }
    return found_nodes;
}

fn extract_switch_link_speeds(paragraph: &Vec<String>) -> Vec<String> {
    let mut my_paragraph = paragraph.clone();
    let mut link_speeds: Vec<String> = Vec::new();

    for line in my_paragraph {
        let device_code: String = line.substring(line.find("\"").unwrap_or(line.len()),line.find("\"").unwrap_or(line.len())+1).to_string();
        if line.contains("[") && device_code == "S" {
            let last_word = line.split(' ').last().unwrap();
            link_speeds.push(last_word.to_string());
        }
    }
    return link_speeds;
}

// struct Chunk {
//     nodes: vec!<vec!<Nodes>>,
//     switches: vec!<Switch>,
// }

// impl Chunk {
//     pub fn chunk_builder(paragraph: &Vec<String>) {
//         Chunk {
//             nodes: extract(paragraph),
//             switches: get_total_switches(paragraph),
//         }
//     }
// }

fn main() {
    let args = Args::parse();
    println!("output: {:?}", args.output);

    let file = get_paragraphs("./ibnetdiscover2023-01-02-18-29-2.txt");

    for &paragraph in &file.clone() {
        let Switch = Switch::build_switch(&paragraph);
        //let Chunk = Chunk::build_chunk(&paragraph);
    }
}

/* get_paragraphs: start from read_lines and then save lines into a vector */
fn get_paragraphs(filename: &str) -> Vec<Vec<String>> {
    // File ibnetdiscover2023-01-02-18-29-2.txt must exist in the current path
    let mut txtfile = vec!();
    let mut templines = vec!();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String

        for line in lines.flatten() {
            if line.trim().is_empty() {
                // New paragraph                                (Paragraph 1)        (Paragraph 2)
                txtfile.push(templines);   // File looks like: [ [line1, line2, ...], [line1, line2, ...], ... ]
                templines.clear();
            } else {
                templines.push(line);
            }
        }
    }
    return txtfile.into();    // paragraph is removed after this function has finished running to preserve mem
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<F>(filename: F) -> io::Result<io::Lines<io::BufReader<File>>>
where F: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
