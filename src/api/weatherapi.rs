use crate::api::Provider;

pub struct Weatherapi {
    pub(crate) api_key: String,
}

impl Provider for Weatherapi {
    fn get_weather(&self, address: &str, date: Option<&str>) -> Result<String, String> {
        Ok(format!(
            "hello from Weatherapi get weather : {} {} api_key = {}",
            address,
            date.unwrap_or_default(),
            self.api_key
        ))
    }
}
