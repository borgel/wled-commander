
use crate::config::Device;
use crate::wled_types::*;

use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
struct LiveInfo {
   effects: HashSet<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Wled {
   // ip, hostname, whatever
   ip: String,
   from_config: Device,

   // info loaded from device
   loaded: Option<LiveInfo>,
}
impl Wled {
   pub fn new(cfg: Device) -> Self {
      Wled {
         ip: sanitize_ipname(&cfg.ip_name),
         // snatch the entire config
         from_config: cfg,
         loaded: None
      }
   }
   // TODO make all of this async
   pub fn init(& mut self) -> Result<(), Box<dyn std::error::Error>> {
      let info = format!("http://{}/json/info", self.ip);
      println!("getting {}", info);
      let resp = reqwest::blocking::get(&info)?
         .json::<Info>()?;
      println!("{:#?}", resp);

      // TODO set params like sync master

      // get the list of effects this device supports
      let efx = format!("http://{}/json/effects", self.ip);
      let loaded_effects = reqwest::blocking::get(&efx)?
         .json::<Effects>()?;

      self.loaded = Some(LiveInfo {
         effects: loaded_effects,
      });

      Ok(())
   }

   /*
   fn apply effect. take brightness, etc?

   fn set boot config
      preset rotation with linger brightness
      */

   /*
   TODO
   fn do_json<T>(&self, opp: JsonApi) -> Result<Box<T>, Box<dyn std::error::Error>> {
      call opp, inflate a T
   }
   */
}

fn sanitize_ipname(n: &String) -> String {
   // TODO remove leading and trailing crap, other cleanup
   let mut o = n.to_string();
   o = o.split_whitespace().collect();
   o
}
