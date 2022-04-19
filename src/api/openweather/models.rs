use serde_derive::{Deserialize, Serialize};

// this struct holds the json response from the open weather api
// each struct represents a json object in the response payload
#[derive(Serialize, Deserialize, Debug)]
pub struct OpenweatherData {
    pub coord: Coord,
    pub weather: Weather,
    pub base: String,
    pub main: Temps,
    pub visibility: i32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i32,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32,
    sys: Sys,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    pub id: i32,
    pub main: String,
    pub description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temps {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}

impl OpenweatherData {
    pub fn current_to_text(&self) -> String {
        format!(
            "Temperature: {:.1}°F\n\
             Wind: {:.0} miles/hr\n\
             Humidity: {}%\n\
             Description: {}",
            self.main.feels_like, self.wind.speed, self.main.humidity, self.weather.details.main
        )
    }
}
