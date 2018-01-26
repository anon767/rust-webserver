
extern crate serde;
extern crate serde_json;


use file_handler::*;
use self::serde_json::Error;


#[derive(Serialize, Deserialize)]
pub struct ConfigDocument {
   pub threads: usize,
   pub ip: String,
   pub port: u32,
   pub document_root: String,
}

pub fn parse_config(content: &String) -> Result<(ConfigDocument), Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = content;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let config: ConfigDocument = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("loading Shit with ip {}:{} \n {} threads \n at {} ", config.ip, config.port, config.threads, config.document_root);

    Ok(config)
}

pub fn parse_file(path: &String) -> ConfigDocument {
    let content: String = read_file(path).unwrap();
    let config: Result<(ConfigDocument), Error> = parse_config(&content);
    config.unwrap()
}