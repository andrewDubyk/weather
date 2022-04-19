mod openweather;
mod weatherapi;

use self::openweather::provider::Openweather;
use self::weatherapi::provider::Weatherapi;

pub trait Provider {
    fn get_weather(&self, address: &str, date: Option<&str>) -> Result<String, String>;
}

pub fn new_provider(name: String, api_key: String) -> Box<dyn Provider> {
    match name.to_lowercase().as_str() {
        "openweather" => Box::new(Openweather { api_key }),
        "weatherapi" => Box::new(Weatherapi { api_key }),
        _ => panic!("Provider {} is not supported", name),
    }
}
