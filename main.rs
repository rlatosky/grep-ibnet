use std::collections::HashMap;
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

// Nodes include [H] AND [S] - anything below a MAIN Switch
#[derive(Clone, Debug)]
struct Node {
    node_type: String,
    hostname_or_model: String,
    uid: String,
    //           portnum,          uid   link speed
    ports: HashMap<String, HashMap<String,String>>,
}

impl Node {
    pub fn build_node(line: &String) -> Node {
        return Node {
            node_type: extract_node_type(line),
            hostname_or_model: extract_node_hostname(line),
            uid: extract_node_uid(line),
            ports: extract_node_ports(line),
        }
    }
}

fn extract_node_type(line: &String) -> String {
    let start_index = line.find("\"").unwrap_or(line.len());
    let uid = line.substring(start_index+1, start_index+2);
    if uid == "S" {
        return "Switch".to_string();
    }
    else {
        return "Host".to_string();
    }
}

fn extract_node_hostname(line: &String) -> String {
    let desired_string: Vec<&str> = line.split("#").collect();
    let start_index = desired_string[1].find("\"").unwrap_or(desired_string[1].len());
    let end_index = desired_string[1].rfind("\"").unwrap_or(desired_string[1].len());
    let mut hostname = desired_string[1].substring(start_index+1, end_index);
    if hostname.contains(";") {
        hostname = hostname.substring(hostname.find(";").unwrap_or(hostname.len())+1, end_index);
        return hostname.to_string();
    } else if hostname.contains(":") || hostname.contains("SW") || hostname.contains(" ") {
        return hostname.to_string();
    }
    return "<None>".to_string();
}

fn extract_node_uid(line: &String) -> String {
    let mut uids: Vec<String> = Vec::new();

    let start_index = line.find("\"").unwrap_or(line.len());
    let uid = line.substring(start_index+3, start_index+19);
    uids.push(uid.to_string());

    // start_index = line.find("(").unwrap_or(line.len());
    // end_index = line.find(")").unwrap_or(line.len());
    // uid = line.substring(start_index+1, end_index);
    // uids.push(uid.to_string());

    return uid.to_string();
}

fn extract_node_ports(line: &String) ->  HashMap<String, HashMap<String,String>> {
    let found_uid;
    let found_portnum;
    let found_link_speed;

    let mut start_index = line.rfind("[").unwrap_or(line.len());
    let mut end_index = line.rfind("]").unwrap_or(line.len());
    found_portnum = line.substring(start_index+1, end_index);

    start_index = line.find("(").unwrap_or(line.len());
    end_index = line.find(")").unwrap_or(line.len());
    found_uid = line.substring(start_index+1, end_index);

    let last_word = line.split(' ').last().unwrap();
    found_link_speed = last_word.to_string();

    let hash_map_uid_link = HashMap::from([(found_uid.to_string(), found_link_speed.to_string())]);
    let hash_map_portnum_uid_link = HashMap::from([(found_portnum.to_string(),hash_map_uid_link)]);
    return hash_map_portnum_uid_link;
}

#[derive(Clone, Debug)]
struct Switch {
    hostname: String,
    uid: String,
    model: String,
    //           portnum,          uid   link speed
    ports: Vec< HashMap<String, HashMap<String,String>> >,
    //link_speeds: Vec<String>,
}

impl Switch {
    pub fn build_switch(paragraph: &Vec<String>) -> Switch {
        return Switch {
            hostname: extract_switch_hostname(paragraph),
            uid: extract_switch_uid(paragraph),
            model: extract_switch_model(paragraph),
            ports: extract_switch_devices(paragraph),
        }
    }

}

fn extract_switch_hostname(paragraph: &Vec<String>) -> String {
    for line in paragraph {
        if line.contains("Switch") {
            let desired_string: Vec<&str> = line.split("#").collect();
            let start_index = desired_string[1].find("\"").unwrap_or(desired_string[1].len());
            let end_index = desired_string[1].rfind("\"").unwrap_or(desired_string[1].len());
            let mut hostname = desired_string[1].substring(start_index, end_index);
            if hostname.contains(";") {
                hostname = hostname.substring(start_index, end_index);
                return hostname.to_string();
            } else if hostname.contains(":") || hostname.contains("SW") {
                return hostname.to_string();
            }
        }
    }
    return "<None>".to_string();
}

fn extract_switch_uid(paragraph: &Vec<String>) -> String {
    let my_paragraph = paragraph.clone();
    let mut uid = "<None>";
    for line in my_paragraph {
        if line.contains("switchguid") {
            let start_index = line.find("(").unwrap_or(line.len());
            let end_index = line.find(")").unwrap_or(line.len());
            uid = line.substring(start_index+1, end_index);
            println!("UID SUBSTRING: {}", uid);
            return uid.to_string();
        }
    }
    return uid.to_string();
}

fn extract_switch_model(paragraph: &Vec<String>) -> String {
    let my_paragraph = paragraph.clone();
    let model = "<None>";
    for line in my_paragraph {
        if line.contains("Switch")  {
            let desired_string: Vec<&str> = line.split("#").collect();
            let start_index = desired_string[1].find("\"").unwrap_or(desired_string[1].len());
            let end_index = desired_string[1].rfind("\"").unwrap_or(desired_string[1].len());
            let model = desired_string[1].substring(start_index+1, end_index);
            if !model.chars().any(|c| matches!(c, '0'..='9')) {
                return model.to_string();
            }
        }
    }
    return model.to_string();
}
// format as a HashMap(uid, portnum)
// collect devices as a hashmap from '[]'
fn extract_switch_devices(paragraph: &Vec<String>) -> Vec< HashMap<String, HashMap<String,String>> > {
    let mut found_uid;
    let mut found_portnum;
    let mut found_link_speed;
    let mut found_nodes = Vec::new();

    for line in paragraph { // working with String
        let mut start_index: usize;
        let end_index: usize;
        let last_word = line.split(' ').last().unwrap();
        if line.contains("[") {
            start_index = line.find("\"").unwrap_or(line.len());
            found_uid = line.substring(start_index+3, start_index+19);

            // start_index = line.find("(").unwrap_or(line.len());
            // end_index = line.find(")").unwrap_or(line.len());
            // found_uid = line.substring(start_index+1, end_index);

            start_index = line.find("[").unwrap_or(line.len());
            end_index = line.find("]").unwrap_or(line.len());

            found_portnum = line.trim().substring(start_index+1, end_index);
            found_link_speed = last_word.to_string();

            let hash_map_uid_portnum = HashMap::from([(found_uid.to_string(), found_link_speed.to_string())]);
            let hash_map_portnum_uid_link = HashMap::from([(found_portnum.to_string(),hash_map_uid_portnum)]);
            found_nodes.push(hash_map_portnum_uid_link);
        }
    }

    return found_nodes;
    //return HashMap::from([("<None>".to_string(), HashMap::from([("<None>".to_string(), "<None>".to_string())]))]);
}

//////////////////////////////////////////////////////

fn main() {
    let file = get_paragraphs("./ibnetdiscover2023-01-02-18-29-2.txt");
    let mut switches: Vec<Switch> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();

    for paragraph in &file.clone() {
        println!("=============================================================");
        println!("=============================================================");
        println!("=============================================================");
        println!("Paragraph: {:#?}\n", paragraph);

        let switch = Switch::build_switch(&paragraph);
        println!("{:#?}", switch);
        switches.push(switch);
        // Any nodes underneath a root switch:
        for line in paragraph {
            if line.contains("[") {
                let node = Node::build_node(&line);
                println!("{:#?}", node);
                nodes.push(node);
            }
        }
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
                txtfile.push(templines.clone());   // File looks like: [ [line1, line2, ...], [line1, line2, ...], ... ]
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
