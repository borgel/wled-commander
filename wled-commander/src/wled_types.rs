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
pub struct InfoLeds {
   count: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
   leds: InfoLeds,
   name: String,
   udpport: u32,
   ver: String,
}

pub type Effects = HashSet<String>;

