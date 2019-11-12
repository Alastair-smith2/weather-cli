This is a Rust CLI app that allows you to enter a location and find out the weather of the location entered.

You are able to find out either about the current weather information or 5 day forecast, the result is also written to a `weather.txt` file.

Setup -

`cargo build` to build the application and download dependencies

Commands -

For the current weather - `cargo run — -l your_location current`
For the 5 day forecast - `cargo run — -l your_location forecast`
