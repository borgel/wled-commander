mod config;
mod wled;
mod wled_types;

use config::*;
use wled::*;

use std::path::PathBuf;
use std::fs::File;
use std::collections::HashMap;
use std::io::Read;
use log::{info, error};

type ConfigFile = HashMap<String, TopLevel>;

fn main() {
   env_logger::init();
   let p = PathBuf::from("house.yaml");
   let cfg = match load_config(&p) {
      Ok(c) => c,
      _ => std::process::exit(-1),
   };

   // split the loaded config file into Devices and Configs
   let mut config: Option<Config> = None;
   let mut controllers: HashMap<String, Wled> = HashMap::new();
   for (name, node) in cfg.into_iter() {
      match node {
         TopLevel::Config(c) => {
            config = Some(c);
         },
         TopLevel::Device(d) => {
            controllers.insert(name, Wled::new(d));
            ()
         },
      }
   }
   if config == None {
      error!("No config file found");
      std::process::exit(-1);
   }
   // now we have a single non Option config!
   let config = config.unwrap();

   info!("Devices {:?}", controllers);
   info!("Config {:?}", config);

   // init everything
   for c in controllers.values_mut() {
      let r = c.init();
      if let Err(e) = r {
         println!("Init failed: {:?}", e);
      }
   }

   // FIXME rm
   println!("done");
}

fn load_config(path: &PathBuf) -> Result<ConfigFile, ()>  {
   let path = PathBuf::from(path);
   let mut f = match File::open(path) {
      Err(_) => return Err(()),
      Ok(file) => file,
   };

   let mut s = String::new();
   f.read_to_string(&mut s).unwrap();

   let inflated: ConfigFile = match serde_yaml::from_str(&s) {
      Ok(inf) => inf,
      Err(e) => {
         // this happens if files are empty, malformed, etc
         println!("failed to inflate: {} from {:?}", e, &f);
         return Err(())
      },
   };

   //println!("inflated info {:#?}\n\n", &inflated);
   Ok(inflated)
}
