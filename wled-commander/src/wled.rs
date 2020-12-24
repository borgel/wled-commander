
use crate::config;
use crate::wled_types;
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
   from_config: config::Device,

   // info loaded from device
   loaded: Option<LiveInfo>,
}
impl Wled {
   pub fn new(cfg: config::Device) -> Self {
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

      // TODO set sync master/receive sync
      let syn_cmd = StateCommand {
         udp: Some(StateUdp {
            send: self.from_config.sync_master,
            recv: !self.from_config.sync_master,
         })
      };

      let syn_url = format!("http://{}/json/state", self.ip);
    let syn_response = reqwest::blocking::Client::new()
        .post(&syn_url)
        .json(&syn_cmd)
        .send()?;
    println!("Response status {}", syn_response.status());

      // get the list of effects this device supports
      let efx = format!("http://{}/json/effects", self.ip);
      let loaded_effects = reqwest::blocking::get(&efx)?
         .json::<Effects>()?;

      self.loaded = Some(LiveInfo {
         effects: loaded_effects,
      });

      // FIXME rm
      println!("initial state: {:#?}", self.get_state());

      Ok(())
   }

   pub fn set_config(&self, cfg: &config::Config) -> Result<(), Box<dyn std::error::Error>> {
      // given the config, build and apply presets and playlists

      // TODO set brightness
      // TODO set presets
      // TODO set preset group progression

      Ok(())
   }

   fn get_state(&self) -> Result<State, Box<dyn std::error::Error>> {
      let st = format!("http://{}/json/state", self.ip);
      Ok(reqwest::blocking::get(&st)?.json::<wled_types::State>()?)
   }

   // API docs here
   // https://github.com/Aircoookie/WLED/wiki/HTTP-request-API
   // https://github.com/Aircoookie/WLED/wiki/JSON-API

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
