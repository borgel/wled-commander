use crate::config::Device;

//num? or array of allowed effect names
//static NUMBERS: &'static [i32] = &[1, 2, 3, 4, 5];

#[derive(Clone, Debug, PartialEq)]
pub struct Wled {
   from_config: Device,
}
impl Wled {
   pub fn new(cfg: Device) -> Self {
      Wled {
         // snatch the entire config
         from_config: cfg,
      }
   }
   pub async fn init(&self) -> Result<(), ()> {
      let body = reqwest::get("https://www.rust-lang.org")
         .await.unwrap()
         .text()
         .await.unwrap();

      println!("body = {:?}", body);
      Ok(())
   }
   /*
   fn init
         connect to controller, confirm there, set params like sync master

   fn apply effect. take brightness, etc?

   fn set boot config
      preset rotation with linger brightness
      */
}
