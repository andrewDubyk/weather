use super::models::WeatherapiData;
use crate::api::Provider;

use reqwest::Url;

const WEATHERAPI_BASE_API_URI: &str = "https://api.weatherapi.com/v1";

/// Weatherapi provider struct
pub struct Weatherapi {
    /// API_KEY for provider services
    pub(crate) api_key: String,
}

/// Weatherapi provider internal implementation
impl Weatherapi {
    /// Returns String with fetched weather data for current date or error details
    ///
    /// # Arguments
    ///
    /// * `address` - Location where to get weather information
    ///
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

        Ok(data.current_to_text())
    }
}

/// Provider interface implementation for Weatherapi provider
impl Provider for Weatherapi {
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
