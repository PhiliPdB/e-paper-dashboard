use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct OpenWeatherMapResponse {
    pub lat: f32,
    pub lon: f32,
    pub timezone: String,
    pub timezone_offset: i32,
    pub current: WeatherReportCurrent,
    pub daily: Vec<Daily>,
}

#[derive(Debug, Deserialize)]
pub struct WeatherReportCurrent {
    pub dt: usize,
    pub sunrise: usize,
    pub sunset: usize,
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: i32,
    pub humidity: i32,
    pub dew_point: f64,
    pub clouds: i32,
    pub uvi: f64,
    pub visibility: i32,
    pub wind_speed: f64,
    pub wind_gust: Option<f64>,
    pub wind_deg: i32,
    pub rain: Option<Precipitation>,
    pub snow: Option<Precipitation>,
    pub weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct Precipitation {
    #[serde(rename="1h")]
    pub one_hour: f64,
}

#[derive(Debug, Deserialize)]
pub struct Daily {
    pub dt: usize,
    pub sunrise: usize,
    pub sunset: usize,
    pub moonrise: usize,
    pub moonset: usize,
    pub moon_phase: f64,
    pub temp: DailyTemp,
    pub feels_like: DailyFeelsLike,
    pub pressure: i32,
    pub humidity: i32,
    pub dew_point: f64,
    pub wind_speed: f64,
    pub wind_gust: Option<f64>,
    pub wind_deg: i32,
    pub clouds: i32,
    pub uvi: f64,
    /// Probability of precipitation
    pub pop: f64,
    pub rain: Option<f64>,
    pub snow: Option<f64>,
    pub weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
pub struct DailyTemp {
    pub morn: f64,
    pub day: f64,
    pub eve: f64,
    pub night: f64,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Deserialize)]
pub struct DailyFeelsLike {
    pub morn: f64,
    pub day: f64,
    pub eve: f64,
    pub night: f64,
}
