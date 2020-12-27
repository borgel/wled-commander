/*
 * Describe all the structures used in the config files
 */

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

type SegmentGroups = Vec<String>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Segment {
   pub start: u32,
   pub end: u32,
   // bools' default is false
   #[serde(default)]
   pub mirror: bool,
   #[serde(default)]
   pub reverse: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
   // IP or hostname
   pub ip_name: String,
   pub segments: HashMap<String, Segment>,
   pub segment_groups: Option<HashMap<String, SegmentGroups>>,
   #[serde(default)]
   #[serde(rename = "syncMaster")]
   pub sync_master: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preset {
   pub segments: HashMap<String, Vec<String>>,
   pub effect: String,
   pub effect_intensity: Option<u8>,
   pub effect_speed: Option<u8>,
   pub color1: Option<u32>,
   pub color2: Option<u32>,
   pub color3: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
   pub brightness: u32,
   pub preset_linger: u32,

   pub presets: HashMap<String, Preset>,
}

// the top level of the file
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TopLevel {
   Device(Device),
   Config(Config),
}

