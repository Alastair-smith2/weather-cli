use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Temperature {
    pub temp: f32,
    pub pressure: f32,
    pub humidity: i32,
    pub temp_min: f32,
    pub temp_max: f32,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The temperature - {}, the humidity -  {}, the minimum temperature - {}, the maximum temperature - {}",
         self.temp, self.humidity, self.temp_min, self.temp_max)
    }
}
