// describe the types used by the WLED JSON API

use serde::{Serialize, Deserialize};
use std::collections::{HashMap};

use raster::Color;

use crate::config;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateUdp {
   pub send: bool,
   pub recv: bool,
}

// FIXME combine with optional one
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct State {
   on: bool,
   #[serde(rename = "ps")]
   current_preset: i32,
   #[serde(rename = "udpn")]
   upd: StateUdp,
   #[serde(rename = "pl")]
   pub current_playlist: i32,
   #[serde(rename = "seg")]
   pub segments: Option<Vec<Segment>>,
}

// this is how you extend an external type to add a new API
trait ColorToArrExt {
   fn to_array(c: &Color) -> [u8; 3];
}
impl ColorToArrExt for raster::Color {
   fn to_array(c: &Color) -> [u8; 3] {
      [c.r, c.g, c.b]
   }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Segment {
   // omitted ID
   pub start: u32,
   pub stop: u32,
   // omitted len, which is inferred from stop
   #[serde(rename = "col")]
   pub colors: [[u8; 3]; 3],
   #[serde(rename = "fx")]
   pub effect_id: u32,
   #[serde(rename = "ix")]
   pub effect_intensity: u8,
   #[serde(rename = "sx")]
   pub effect_speed: u8,

   #[serde(rename = "rev")]
   pub reverse: bool,
   #[serde(rename = "mi")]
   pub mirror: bool,
}
#[derive(Clone, Debug)]
pub struct SegmentExtras {
   pub colors: (Color, Color, Color),
   pub effect_id: u32,
   pub effect_intensity: u8,
   pub effect_speed: u8,
}
impl Segment {
   pub fn new(other: &config::Segment, extras: Option<&SegmentExtras>) -> Self {
      let default_extras = SegmentExtras {
         colors: (Color::black(), Color::black(), Color::black()),
         effect_id: 0,
         effect_intensity: 0,
         effect_speed: 0,
      };
      let clean_extras = extras.to_owned().unwrap_or(&default_extras);
      let color_arr = [
         Color::to_array(&clean_extras.colors.0),
         Color::to_array(&clean_extras.colors.1),
         Color::to_array(&clean_extras.colors.2)
      ];

      Segment {
         start: other.start,
         stop: other.end,
         reverse: other.reverse,
         mirror: other.mirror,
         colors: color_arr,
         effect_id: clean_extras.effect_id,
         effect_intensity: clean_extras.effect_intensity,
         effect_speed: clean_extras.effect_speed,
      }
   }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateCommand {
   // fill in just the fields you want to send as the command
   #[serde(skip_serializing_if = "Option::is_none")]
   #[serde(rename = "udpn")]
   pub udp: Option<StateUdp>,
   #[serde(skip_serializing_if = "Option::is_none")]
   #[serde(rename = "bri")]
   pub brightness: Option<u32>,
   #[serde(skip_serializing_if = "Option::is_none")]
   #[serde(rename = "psave")]
   pub set_preset: Option<u32>,
   #[serde(skip_serializing_if = "Option::is_none")]
   #[serde(rename = "pl")]
   pub current_playlist: Option<i32>,
   #[serde(skip_serializing_if = "Option::is_none")]
   #[serde(rename = "seg")]
   pub segments: Option<Vec<Segment>>,
   #[serde(skip_serializing_if = "Option::is_none")]
   #[serde(rename = "playlist")]
   pub playlist: Option<Playlist>,
}

// map effect name to index
pub type Effects = HashMap<String, u32>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
   leds: InfoLeds,
   name: String,
   udpport: u32,
   #[serde(rename = "ver")]
   version: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoLeds {
   count: u32,
   // other fields we don't care about
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlaylistDuration {
   Single(u32),
   Multiple(Vec<u32>),
}
impl std::default::Default for PlaylistDuration {
   fn default() -> Self { Self::Single(0) }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlaylistTransition {
   Single(u32),
   Multiple(Vec<u32>),
}
impl std::default::Default for PlaylistTransition {
   fn default() -> Self { Self::Single(0) }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Playlist {
   #[serde(rename = "ps")]
   pub presets: Vec<u32>,
   #[serde(rename = "dur")]
   //pub duration: PlaylistDuration,
   pub duration: u32,
   #[serde(rename = "transition")]
   //pub transition_time: PlaylistTransition,
   pub transition_time: u32,
   #[serde(skip_serializing_if = "Option::is_none")]
   pub repeat: Option<u32>, // 0 for indefinite
   #[serde(skip_serializing_if = "Option::is_none")]
   pub end: Option<u32>, // what preset to apply if stops repeating
}


