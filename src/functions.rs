// This module defines all helper functions for the main program


use std::io::Read;
use std::str::from_utf8;
use std::error::Error;


pub fn get_nodes(regex: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut config_buf: Vec<u8> = vec![];
    let mut _path = std::env::var("HOME")?;
    _path.push_str("/.ssh/config");
    let mut ssh_conf_file = std::fs::File::open(_path)?;

    ssh_conf_file.read_to_end(&mut config_buf)?;
    let config_str: String = String::from(from_utf8(&config_buf)?);

    let hostname_regex = regex::Regex::new(&format!("[^#]Host {}", &regex))?;
    let regex_iter = hostname_regex.find_iter(&config_str);
    let mut nodes: Vec<String> = vec![];
        for host in regex_iter {
            nodes.push(String::from(host.as_str().split_once(" ").unwrap().1));
        }
    Ok(nodes)
}