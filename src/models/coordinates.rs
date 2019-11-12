use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Coordinates {
    pub lon: f32,
    pub lat: f32,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Latitude -  {}, Longtitude - {}", self.lat, self.lon)
    }
}
