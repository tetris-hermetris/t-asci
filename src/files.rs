#[path = "./config.rs"] mod config;

use config::WORKING_PATH;

use std::fs;

pub fn read_spores() -> Vec<String>{
    let paths = fs::read_dir(WORKING_PATH).unwrap();
    
  let names =
  paths.filter_map(|entry| {
    entry.ok().and_then(|e|
      e.path().file_name()
      .and_then(|n| n.to_str().map(|s| String::from(s)))
    )
  }).collect::<Vec<String>>();
  names
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_lookup() {
        let lookup =  read_spores();
        println!("{:?}", lookup)
    }
}
