use serde::Deserialize;
use super::city::City;
use super::temperature::Temperature;
use super::weather::Weather;

#[derive(Deserialize, Debug)]
pub struct ForecastWeather {
    pub main: Temperature,
    pub weather: Vec<Weather>,
    pub dt_txt: String,
}

#[derive(Deserialize, Debug)]
pub struct ForecastResponse {
    cod: String,
    pub city: City,
    pub list: Vec<ForecastWeather>,
}
