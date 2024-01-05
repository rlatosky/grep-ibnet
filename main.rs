use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

/*
    Some Information:
    ->
    https://stackoverflow.com/questions/43820696/how-can-i-find-the-index-of-a-character-in-a-string-in-rust
 */
// clap library for argument management
// take from std::in

// 

struct Nodes {
    hostname: String,
    uid: String,
    linkspeed: i32,
}

impl Nodes {
    pub fn build_node()
}

struct Switch {
    guid: String,
    model: String,
    // Anything plugged in - Could be switch or node
    //                uid    portnum
    devices: HashMap<String, String> 
}

impl Switch {
    pub fn build_switch(guid: String, model: String, devices: HashMap<String, String>) -> Switch {
        Switch {
            guid: extract_guid(paragraph: &str),
            model: extract_model(paragraph: &str),
            nodes: nodes(paragraph: &str),
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
    
}

fn extract_nodes(paragraph: &str) {
    if paragraph.contains("farm") {
        let len_farm_hostname = 6;  // We assume farm hostname only contains six chars
        let farm_hostname_index = paragraph.find("farm")   // Get index where 'farm' is at
        let hostname: String = paragraph.chars().skip(farm_hostname_index).take(len_farm_hostname).collect();
        .find("#");
        .find("\"");
        .find("\"").next();
        println!("SUBSTRING: {}", h    fn extract_guid(paragraph: &str) {
        // get Switch IDs
        if paragraph.contains("switchguid") {
            let len_farm_hostname = 16;  // We assume farm hostname only contains six chars
            let farm_hostname_index = paragraph.find("switchguid") + 1  // Get index where 'farm' is at
            let hostname: String = paragraph.chars().skip(farm_hostname_index + 1).take(len_farm_hostname).collect();
            println!("SUBSTRING: {}", hostname);
        }
    }
    
    fn extract_model(paragraph: &str) {
        
    }
    
    fn extract_nodes(paragraph: &str) {
        if paragraph.contains("farm") {
            let len_farm_hostname = 6;  // We assume farm hostname only contains six chars
            let farm_hostname_index = paragraph.find("farm")   // Get index where 'farm' is at
            let hostname: String = paragraph.chars().skip(farm_hostname_index).take(len_farm_hostname).collect();
            .find("#");
            .find("\"");
            .find("\"").next();
            println!("SUBSTRING: {}", hostname);
        }
    }ostname);
    }
}

struct Chunks {
    nodes: vec<Nodes>,
    switches: vec<Switch>,
}

trait ChunkBuilder{
    pub fn chunkBuilder() {
        Switch.build_switch()
    }
}

fn main() {
    let args: ; // Clap Args
    let paragraphs = get_paragraphs();
    let switches = get_switches(paragraphs.clone());    // .clone() makes a copy and follows memory safety
    let nodes = get_nodes(paragraphs.clone());
    // clone args to memory safe
}

fn get_switches(paragraphs: Vec<String>) {
    let mut my_paragraph = paragraph.clone();
}

fn get_paragraphs() -> Vec<String> {
    // File ibnetdiscover2023-01-02-18-29-2.txt must exist in the current path
    if let Ok(lines) = read_lines("./ibnetdiscover2023-01-02-18-29-2.txt") {
        // Consumes the iterator, returns an (Optional) String
        Vec<String> paragraphs;
        Vec<String> lines;
        
        for line in lines.flatten() {
            if line.trim().is_empty() {
                // New paragraph
                paragraphs.append(lines);
                lines.clear();
            } else {
                lines.append(line);
            }
        }
    }
    paragraphs.into()   // paragraphs is removed after this function has finished running to preserve mem
}
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<F>(filename: F) -> io::Result<io::Lines<io::BufReader<File>>>
where F: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
