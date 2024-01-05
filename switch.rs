impl Switch {
    pub fn build_switch(guid: String, model: String, nodes: Vec) -> Switch {
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
    println!("SUBSTRING: {}", hostname);
}
}
