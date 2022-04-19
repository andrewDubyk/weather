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

impl WeatherapiData {
    pub fn current_to_text(&self) -> String {
        format!(
            "Current temperature: {:.1}°F\n\
             Wind: {:.0} miles/hr\n\
             Humidity: {}%",
            self.current.temp_c, self.current.wind_mph, self.current.humidity
        )
    }
}
