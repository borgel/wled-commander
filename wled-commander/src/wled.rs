use crate::config;
use crate::wled_types;
use crate::wled_types::*;

use std::collections::{HashMap};
use std::default::Default;

use raster::Color;

#[derive(Clone, Debug, PartialEq)]
struct LiveInfo {
   effects: Effects,
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
         .json::<Vec<String>>()?;

      // reshape to a hashmap of effects
      let mut effect_map: Effects = HashMap::new();
      for (i, e) in loaded_effects.iter().enumerate() {
         effect_map.insert(e.to_lowercase(), i as u32);
      }

      // FIXME rm
      println!("effects\n{:?}", effect_map);

      self.loaded = Some(LiveInfo {
         effects: effect_map,
      });

      // FIXME rm
      println!("initial state: {:#?}", self.get_state());

      Ok(())
   }

   pub fn set_config(&self, cfg: &config::Config) -> Result<(), Box<dyn std::error::Error>> {
      // given the config, build and apply presets and playlists

      // build brightness
      let scaled_brightness = (cfg.brightness as f32 / 100.0) * 255.0;

      // FIXME rm
      /*
      // build segments
      let mut segs: Vec<wled_types::Segment> = Vec::new();
      for s in self.from_config.segments.values() {
         segs.push(wled_types::Segment::new(&s));
      }
      */

      // set everything at once
      let big_cmd = StateCommand {
         brightness: Some(scaled_brightness as u32),
         //segments: Some(segs),
         ..Default::default()
      };
      let r = self.set_state(&big_cmd)?;

      // FIXME rm
      println!("Set resulting state: {:#?}", r);
      println!("State after set: {:#?}", self.get_state());

      Ok(())
   }

   // TODO take a different preset structure that's missing segments
   pub fn set_preset(&self, slot: u32, preset: &config::Preset, segments_in_preset: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
      let effect_intensity = preset.effect_intensity.unwrap_or(128);
      let effect_speed = preset.effect_speed.unwrap_or(128);

      // build color tuple
      let c1 = format!("#{:x}", preset.color1.unwrap_or(0x0));
      let c1 = Color::hex(&c1).unwrap();
      let c2 = format!("#{:x}", preset.color2.unwrap_or(0x0));
      let c2 = Color::hex(&c2).unwrap();
      let c3 = format!("#{:x}", preset.color3.unwrap_or(0x0));
      let c3 = Color::hex(&c3).unwrap();
      let colors = (c1, c2, c3);

      let extras = SegmentExtras {
         colors: colors,
         // silently convert unknown effects into solid color
         effect_id: self.get_effect_id(&preset.effect).unwrap_or(0),
         effect_intensity: effect_intensity,
         effect_speed: effect_speed
      };

      // filter the incoming list of segments based on Self's list of segments, get those
      // segments from Self, then for each construct a wled_types::Segment and build a Vec
      let segs: Vec<Segment> = segments_in_preset.iter()
         .filter(|s| self.from_config.segments.contains_key(*s))
         // we've filtered for segments that exist, so this unwrap is safe
         .map(|s| self.from_config.segments.get(s).unwrap())
         .map(|s| wled_types::Segment::new(s, Some(&extras)))
         .collect();

      // with one call, set these segments and save it as a preset slot
      self.set_state(& StateCommand {
         segments: Some(segs),
         set_preset: Some(slot),
         ..Default::default()
      })?;

      Ok(())
   }

   fn get_effect_id(&self, name: &str) -> Result<u32, ()> {
      if let Some(loaded) = &self.loaded {
         if let Some(idx) = loaded.effects.get(name) {
            return Ok(*idx);
         }
      }
      Err(())
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
