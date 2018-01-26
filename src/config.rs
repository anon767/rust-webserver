
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
    let data = content;

    let config: ConfigDocument = serde_json::from_str(data)?;

    println!("loading Shit with ip {}:{} \n {} threads \n at {} ", config.ip, config.port, config.threads, config.document_root);

    Ok(config)
}

pub fn parse_file(path: &String) -> ConfigDocument {
    let content: String = read_file(path).unwrap();
    let config: Result<(ConfigDocument), Error> = parse_config(&content);
    config.unwrap()
}
