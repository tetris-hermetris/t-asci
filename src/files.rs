#[path = "./config.rs"] mod config;
#[path = "./spores.rs"] mod spores;

use config::WORKING_PATH;
use spores::{Spores, Spore};
use ron::ser::{to_writer_pretty, PrettyConfig};
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

pub fn write_spores(spores: Spores) {
  for spore in spores.clone().into_iter() {
    let filename = WORKING_PATH.to_string() + "/" + &spore.tag + "-" + &spore.id.to_string() + ".spore";
    let file = fs::File::create(filename).unwrap();
    to_writer_pretty(file, &spore, PrettyConfig::new()).unwrap();
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_lookup() {
        let lookup =  read_spores();
        println!("{:?}", lookup)
    }

    #[test]
    fn test_write_spores() {
      let mut spores = Spores::new();
      spores.insert_spore("New spore".to_string());
      let files_cnt_before = read_spores().len();
      write_spores(spores);
      let files_cnt_after = read_spores().len();
      assert_eq!(files_cnt_before, files_cnt_after - 1)
    }
}
