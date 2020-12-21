mod config;

use config::*;

use std::path::PathBuf;
use std::fs::File;
use std::collections::HashMap;
use std::io::Read;

type ConfigFile = HashMap<String, TopLevel>;
//type ConfigFile = Vec<TopLevel>;

fn main() {
    println!("Hello, world!");

    let p = PathBuf::from("house.yaml");
    load_config(&p);
}

fn load_config(path: &PathBuf) -> Result<ConfigFile, ()>  {
   let path = PathBuf::from(path);
   let mut f = match File::open(path) {
      Err(_) => return Err(()),
      Ok(file) => file,
   };

   println!("Opened {:?}", &f);

   let mut s = String::new();
   f.read_to_string(&mut s).unwrap();

   let inflated: ConfigFile = match serde_yaml::from_str(&s) {
      Ok(inf) => inf,
      Err(e) => {
         // this happens if files are empty, malformed, etc
         println!("failed to inflate: {}", e);
         return Err(())
      },
   };

   println!("inflated info {:#?}\n\n", &inflated);
   Ok(inflated)
}
