mod openweather;
mod weatherapi;

pub struct Provider {
}

pub trait ProviderApi {
    pub fn get_weather(&self, address: &str, date: Option<&str>) -> Result<String, String>;
}

