mod openweather;
mod weatherapi;

pub trait Provider {
    fn get_weather(&self, address: &str, date: Option<&str>) -> Result<String, String>;
}

pub fn new_provider(name: String, api_key: String) -> Box<dyn Provider> {
    match name.to_lowercase().as_str() {
        "openweather" => Box::new(openweather::Openweather { api_key }),
        "weatherapi" => Box::new(weatherapi::Weatherapi { api_key }),
        _ => panic!("Provider {} is not supported", name),
    }
}
