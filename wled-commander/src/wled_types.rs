// describe the types used by the WLED JSON API

use serde::{Serialize, Deserialize};
use std::collections::HashSet;

// FIXME mv? rename?
// exposed JSON APIs
pub enum JsonApi {
   Info,
   Effects,
   State
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateUdp {
   pub send: bool,
   pub recv: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct State {
   on: bool,
   #[serde(rename = "ps")]
   current_preset: u32,
   #[serde(rename = "udpn")]
   upd: StateUdp,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateCommand {
   // fill in just the fields you want to send as the command
   #[serde(skip_serializing_if = "Option::is_none")]
   #[serde(rename = "udpn")]
   pub udp: Option<StateUdp>,
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


