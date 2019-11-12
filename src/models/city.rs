use serde::Deserialize;
use super::coordinates::Coordinates;

#[derive(Deserialize, Debug)]
pub struct City {
    pub name: String,
    pub coord: Coordinates,
    pub country: String,
}
