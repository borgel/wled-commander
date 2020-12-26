
use crate::config;
use crate::wled_types;
use crate::wled_types::*;

use std::collections::{HashMap, HashSet};
use std::default::Default;

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

      // set sync master/receive sync
      let syn_cmd = StateCommand {
         udp: Some(StateUdp {
            send: self.from_config.sync_master,
            recv: !self.from_config.sync_master,
         }),
         ..Default::default()
      };
      self.set_state(&syn_cmd)?;

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

      // build brightness
      let scaled_brightness = (cfg.brightness as f32 / 100.0) * 255.0;

      // FIXME TODO
      // iterate through presets
      //    for each preset, figure out what segments are active, set colors and config on the device, and save it in a preset slot

      // build segments
      let mut segs: Vec<wled_types::Segment> = Vec::new();
      for s in self.from_config.segments.values() {
         segs.push(wled_types::Segment::new(&s));
      }

      // set everything at once
      let big_cmd = StateCommand {
         brightness: Some(scaled_brightness as u32),
         segments: Some(segs),
         ..Default::default()
      };
      let r = self.set_state(&big_cmd)?;

      // FIXME rm
      println!("Set resulting state: {:#?}", r);
      println!("State after set: {:#?}", self.get_state());

      Ok(())
   }

   pub fn set_preset(&self, slot: u32, preset: &config::Preset, segment: &str) -> Result<(), Box<dyn std::error::Error>> {
      // FIXME rm
      println!("Set preset slot {} on segment {}", slot, segment);

      // TODO configmr segment exists
      // TODO set preset on segment. load state first to work from?
      Ok(())
   }

   fn get_state(&self) -> Result<State, Box<dyn std::error::Error>> {
      let st = format!("http://{}/json/state", self.ip);
      Ok(reqwest::blocking::get(&st)?.json::<wled_types::State>()?)
   }

   fn set_state(&self, new_state: &StateCommand) -> Result<StateCommand, Box<dyn std::error::Error>> {
      // FIXME info
      println!("Setting state {:#?}", new_state);

      let syn_url = format!("http://{}/json/state", self.ip);
      let response = reqwest::blocking::Client::new()
         .post(&syn_url)
         .json(&new_state)
         .send()?;
      if response.status() == 200 {
         return Ok(response.json::<StateCommand>()?);
      }
      Err(Box::new(response.error_for_status().err().unwrap()))
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
