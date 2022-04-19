use crate::api::Provider;

pub struct Openweather {
    pub(crate) api_key: String,
}

impl Provider for Openweather {
    fn get_weather(&self, address: &str, date: Option<&str>) -> Result<String, String> {
        Ok(format!(
            "hello from Openweather get weather : {} {} {}",
            address,
            date.unwrap_or_default(),
            self.api_key
        ))
    }
}
