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

   for c in controllers.values_mut() {
      let r = c.init();
      if let Err(e) = r {
         println!("Init failed: {:?}", e);
      }
      let r = c.set_config(&config);
      if let Err(e) = r {
         println!("Set config failed: {:?}", e);
      }
   }

   // start the vec of preset IDs in use which we will turn into a playlist
   let mut plist: Vec<u32> = Vec::new();

   // iterate through config presets, slice them by targetted device, and send
   for (idx, pre) in config.presets.values().enumerate() {
      plist.push((idx + 1) as u32);
      for (device_name, segments) in &pre.segments {
         if let Some(c) = controllers.get(device_name) {
            if let Err(e) = c.set_preset(idx as u32, &pre, &segments) {
               error!("Failed to set preset slot {} on {}: {}", idx, device_name, e);
            }
         }
      }
   }
   // now all presets on all devices are configured in the same oreder and with the correct segments

   // FIXME rm
   println!("{:?}", plist);

   // set preset group progression for all devices based on config
   for c in controllers.values() {
      let r = c.set_playlist(
         &plist,
         config.preset_linger,
         1, // transition time
         );

      // TODO some more elegant way to handle this
      if let Err(e) = r {
         error!("Failed to set playlist on {:?}: {}", c, e);
      }
   }
   println!("Setup an effect playlist {} items long", plist.len());
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
