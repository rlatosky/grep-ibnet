struct Node {
    hostname: String,
    uid: String,
    portnum: i32,
    link_speed: String,
}

impl Node {
    pub fn build_node(hostname: String, uid: String, portnum: i32, link_speed: String) {
        Node {
            hostname: extract_hostname(paragraph: &str);
            uid: extract_uid(paragraph: &str);
            portnum: extract_portnum(paragraph: &str);
            link_speed: extract_link_speed(paragraph: &str);
        }
    }
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
    pub fn build_switch(guid: String, model: String, devices: HashMap<String, String>, link_speed: String) -> Switch {
        Switch {
            guid: extract_guid(paragraph: &str),
            model: extract_model(paragraph: &str),
            devices: extract_devices(paragraph: &str),
            link_speed: extract_link_speed(paragraph: &str),
        }
    }

    fn extract_guid(paragraph: &str) -> String {
    // get Switch IDs
        for line in paragraph {
            if paragraph.contains("switchguid") {
                let len_farm_hostname = 16;  // We assume farm hostname only contains six chars
                let farm_hostname_index = paragraph.find("switchguid") + 1  // Get index where 'farm' is at
                let hostname: String = paragraph.chars().skip(farm_hostname_index + 1).take(len_farm_hostname).collect();
                println!("SUBSTRING: {}", hostname);
            }
        }
    }

    fn extract_model(paragraph: &str) -> String {
        if paragraph.contains("Switch") {
            while paragraph.find("#") > paragraph.find("\"") {
                let start_index = paragraph.find("\"");
                let end_index = paragraph.find("\"").next();
            }
            let len_indexes = end_index - start_index;

            // .skip() means skipping TO that index
            let model: String = paragraph.chars().skip(start_index).take(len_indexes).collect();
            println!("SUBSTRING: {}", model);
        }
    }
    // format as a HashMap(uid, portnum)
    // collect devices as a hashmap from '[]'
    fn extract_devices(paragraph: &str) -> String {
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

    fn extract_link_speeds(paragraph: &str) -> Vec<String> {
        Vec<String> link_speeds;
        for line in paragraph {
            let last_word = line.split(' ').last().unwrap();
            link_speeds.push(last_word);
        }
    }
}

struct Chunk {
    nodes: Vec<Nodes>,
    switches: Vec<Switch>,
}

impl Chunk {
    pub fn chunk_builder() {
        Chunk {
            nodes: Vec<Node>,
            switches: Vec<Switch>,
        }
    }
}
)
