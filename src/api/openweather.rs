use crate::api::Provider;

use reqwest::Url;
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

impl ToString for OpenweatherData {
    fn to_string(&self) -> String {
        format!(
            "Temperature: {:.1}°F\n\
             Wind: {:.0} miles/hr\n\
             Humidity: {}%\n\
             Description: {}",
            self.main.feels_like, self.wind.speed, self.main.humidity, self.weather.details.main
        )
    }
}

const OPENWEATHER_BASE_API_URI: &str = "https://api.openweathermap.org/data/2.5";

pub struct Openweather {
    pub(crate) api_key: String,
}

impl Openweather {
    pub fn get_weather_now(&self, address: &str) -> Result<String, String> {
        let url = format!(
            "{}/weather?q={}&appid={}",
            OPENWEATHER_BASE_API_URI, address, self.api_key
        );

        let url = match Url::parse(url.as_str()) {
            Ok(url) => url,
            Err(e) => return Err(format!("Failed to parse url : {}", e)),
        };

        let data = match reqwest::blocking::get(url) {
            Ok(response) => match response.json::<OpenweatherData>() {
                Ok(data) => data,
                Err(e) => return Err(format!("Failed to parse weather data : {}", e)),
            },
            Err(e) => return Err(format!("Failed to fetch weather data : {}", e)),
        };

        Ok(data.to_string())
    }
}

impl Provider for Openweather {
    fn get_weather(&self, address: &str, date: Option<&str>) -> Result<String, String> {
        match date {
            Some(date) => Err(format!(
                "Openweather API doesn't support date forecats for {}",
                date
            )),
            None => self.get_weather_now(address),
        }
    }
}
