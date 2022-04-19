mod openweather;
mod weatherapi;

use self::openweather::provider::Openweather;
use self::weatherapi::provider::Weatherapi;

/// Common trait for all supported porviders
pub trait Provider {
    /// Returns String with fetched weather data or error details
    ///
    /// # Arguments
    ///
    /// * `address` - Location where to get weather information
    /// * `date` - Date is optional, None means now
    ///
    fn get_weather(&self, address: &str, date: Option<&str>) -> Result<String, String>;
}

/// Create instance of requested provider. Terminates if providers is not supported
///
/// # Arguments
///
/// * `name` - Provider name
/// * `api_key` - Provider API_KEY
///
pub fn new_provider(name: String, api_key: String) -> Box<dyn Provider> {
    match name.to_lowercase().as_str() {
        "openweather" => Box::new(Openweather { api_key }),
        "weatherapi" => Box::new(Weatherapi { api_key }),
        _ => panic!("Provider {} is not supported", name),
    }
}
