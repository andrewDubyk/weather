use super::models::OpenweatherData;
use crate::api::Provider;

use reqwest::Url;

const OPENWEATHER_BASE_API_URI: &str = "https://api.openweathermap.org/data/2.5";

/// Openweather provider struct
pub struct Openweather {
    /// API_KEY for provider services
    pub(crate) api_key: String,
}

/// Openweather provider internal implementation
impl Openweather {
    /// Returns String with fetched weather data for current date or error details
    ///
    /// # Arguments
    ///
    /// * `address` - Location where to get weather information
    ///
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

        Ok(data.current_to_text())
    }
}

/// Provider interface implementation for Openweather provider
impl Provider for Openweather {
    /// Trait get_weather implementation
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
