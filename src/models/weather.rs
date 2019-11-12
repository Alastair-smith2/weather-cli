use serde::Deserialize;
use std::fmt;

use super::coordinates::Coordinates;
use super::temperature::Temperature;

#[derive(Deserialize, Debug)]
pub struct Weather {
    id: i32,
    pub main: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    cod: i32,
    pub name: String,
    pub weather: Vec<Weather>,
    pub coord: Coordinates,
    pub main: Temperature,
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The weather - {}", self.description)
    }
}

impl fmt::Display for WeatherResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The weather - {:#?}", self)
    }
}
