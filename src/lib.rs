use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn from_file(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let ret: HashMap<String, String> = HashMap::new();
    let path = Path::new(path);
    let binding = fs::read_to_string(path)?;
    let lines = binding.lines();
    let mut value = HashMap::new();
    //-- collect hashmap from file
    for line in lines {
        match line.split_once(':') {
            Some((key, val)) => {
                let key = key.trim().to_string();
                let val = val.trim().to_string();
                value.insert(key, val);
            }
            _ => return Err("line has no delimiter ':'".into()),
        };
    }
    Ok(ret)
}
