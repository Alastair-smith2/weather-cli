This is a Rust CLI app that allows you to enter a location and find out the weather of the location entered using `api.openweather`

<br/>

You are able to find out either about the current weather information or 5 day forecast, the result is also written to a `weather.txt` file.

# Setup -

Run `cargo build` to build the application and download dependencies

You will also need to create a `.env` file and add your [open weather api key](https://openweathermap.org/api)

The API key is in the format of `API_KEY=YOUR_KEY_HERE`

## Commands -

For the current weather - `cargo run — -l your_location current`
<br/>
For the 5 day forecast - `cargo run — -l your_location forecast`
