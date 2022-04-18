struct OpenweatherImpl {}

impl ProviderApi for OpenweatherImpl {
    pub fn get_weather(&self, address: &str, date: Option<&str>) -> Result<String, String> {}
}
