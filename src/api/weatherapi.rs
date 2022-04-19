use crate::api::Provider;

use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

// this struct holds the json response from the open weather api
// each struct represents a json object in the response payload
#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherapiData {
    pub location: Location,
    pub current: Forecast,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub tz_id: String,
    pub localtime_epoch: u64,
    pub localtime: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub last_updated_epoch: u64,
    pub last_updated: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub is_day: i32,
    pub wind_mph: f64,
    pub wind_kph: f64,
    pub wind_degree: f64,
    pub wind_dir: String,
    pub pressure_mb: f64,
    pub pressure_in: f64,
    pub precip_mm: f64,
    pub precip_in: f64,
    pub humidity: f64,
    pub cloud: f64,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    pub vis_km: f64,
    pub vis_miles: f64,
    pub uv: f64,
    pub gust_mph: f64,
    pub gust_kph: f64,
}

impl ToString for WeatherapiData {
    fn to_string(&self) -> String {
        format!(
            "Current temperature: {:.1}°F\n\
             Wind: {:.0} miles/hr\n\
             Humidity: {}%",
            self.current.temp_c, self.current.wind_mph, self.current.humidity
        )
    }
}

const WEATHERAPI_BASE_API_URI: &str = "https://api.weatherapi.com/v1";

pub struct Weatherapi {
    pub(crate) api_key: String,
}

impl Weatherapi {
    pub fn get_weather_now(&self, address: &str) -> Result<String, String> {
        let url = format!(
            "{}/current.json?q={}&key={}",
            WEATHERAPI_BASE_API_URI, address, self.api_key
        );

        let url = match Url::parse(url.as_str()) {
            Ok(url) => url,
            Err(e) => return Err(format!("Failed to parse url : {}", e)),
        };

        let data = match reqwest::blocking::get(url) {
            Ok(response) => match response.json::<WeatherapiData>() {
                Ok(data) => data,
                Err(e) => return Err(format!("Failed to parse weather data : {}", e)),
            },
            Err(e) => return Err(format!("Failed to fetch weather data : {}", e)),
        };

        Ok(data.to_string())
    }
}

impl Provider for Weatherapi {
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
