
use crate::models::{forecast};
use forecast::{ForecastResponse};

pub fn print_forecast_info(fetch_result: ForecastResponse) {
    let mut count = 0;
    println!("In {:#?}", &fetch_result.city.name);
    for weather_object in &fetch_result.list {
        if count < 5 {
            println!("At {:#?}", &weather_object.dt_txt);
            println!(
                "The weather is {:#?}",
                &weather_object.weather[0].description
            );
            println!(
                "The temperature is {:#?} degrees",
                &weather_object.main.temp
            );
            println!("The humidity is {:#?}%", &weather_object.main.humidity);
            count += 1;
        }
    }
}
