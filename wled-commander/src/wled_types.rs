// describe the types used by the WLED JSON API

use serde::{Serialize, Deserialize};
use std::collections::HashSet;

use crate::config;

// FIXME mv? rename?
// exposed JSON APIs
pub enum JsonApi {
   Info,
   Effects,
   State
}

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
   #[serde(rename = "seg")]
   pub segments: Option<Vec<Segment>>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Segment {
   // omitted ID
   pub start: u32,
   pub stop: u32,
   // omitted len
   // omitted colors

   // need effect, speed, intensity?

   #[serde(rename = "rev")]
   pub reverse: bool,
   #[serde(rename = "mi")]
   pub mirror: bool,
}
impl Segment {
   pub fn new(other: &config::Segment) -> Self {
      Segment {
         start: other.start,
         stop: other.end,
         reverse: other.reverse,
         mirror: other.mirror,
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
   #[serde(rename = "seg")]
   pub segments: Option<Vec<Segment>>,
}

pub type Effects = HashSet<String>;

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlaylistTransition {
   Single(u32),
   Multiple(Vec<u32>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Playlist {
   #[serde(rename = "ps")]
   presets: Vec<u32>,
   #[serde(rename = "dur")]
   duration: PlaylistDuration,
   #[serde(rename = "transition")]
   transition: PlaylistTransition,
   #[serde(skip_serializing_if = "Option::is_none")]
   repeat: Option<u32>, // 0 for indefinite
   #[serde(skip_serializing_if = "Option::is_none")]
   end: Option<u32>, // what preset to apply if stops repeating
}


