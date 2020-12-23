use crate::config::Device;

use std::collections::HashMap;

//num? or array of allowed effect names
//static NUMBERS: &'static [i32] = &[1, 2, 3, 4, 5];

#[derive(Clone, Debug, PartialEq)]
pub struct Wled {
   live: bool,
   from_config: Device,
}
impl Wled {
   pub fn new(cfg: Device) -> Self {
      Wled {
         live: false,
         // snatch the entire config
         from_config: cfg,
      }
   }
   // TODO make all of this async
   pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
      // TODO connect to controller, confirm there, set params like sync master

      let resp = reqwest::blocking::get("https://httpbin.org/ip")?
         .json::<HashMap<String, String>>()?;
      println!("{:#?}", resp);
      Ok(())
   }

   /*
   fn apply effect. take brightness, etc?

   fn set boot config
      preset rotation with linger brightness
      */
}
