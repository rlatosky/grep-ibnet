mod extract_ibnet;
use extract_ibnet::*;

// Clap used for arguments
/*
    Some Information:
    ->
    https://stackoverflow.com/questions/43820696/how-can-i-find-the-index-of-a-character-in-a-string-in-rust
 */
// clap library for argument management
// take from std::in

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// File to look for servers/hosts
    ibnet: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

//////////////////////////////////////////////////////

fn main() {
    let cli = Cli::parse(); // Parse command-line arguments

    if let Some(config_path) = cli.ibnet.as_deref() {
        println!("Value for config: {}", config_path);
        let file = get_paragraphs(config_path);

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

}