use std::fmt;
use std::io;
use reqwest;
use std::error;

#[derive(Debug)]
pub enum WeatherError {
    ParseError,
    Io(io::Error),
    Network(reqwest::Error)
}

impl fmt::Display for WeatherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WeatherError::Io(ref err) => write!(f, "IO error: {}", err),
            WeatherError::Network(ref err) => write!(f, "Network error: {}", err)
        }
    }
}

impl error::Error for WeatherError {
    fn description(&self) -> &str {
        match *self {
            WeatherError::Io(ref err) => err.description(),
            // Normally we can just write `err.description()`, but the error
            // type has a concrete method called `description`, which conflicts
            // with the trait method. For now, we must explicitly call
            // `description` through the `Error` trait.
            WeatherError::Network(ref err) => error::Error::description(err),
        }
    }

     fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            WeatherError::Io(ref err) => Some(err),
            WeatherError::Network(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for WeatherError {
    fn from(err: io::Error) -> WeatherError {
        WeatherError::Io(err)
    }
}

impl From<reqwest::Error> for WeatherError {
    fn from(err: reqwest::Error) -> WeatherError {
        WeatherError::Network(err)
    }
}
