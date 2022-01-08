use embedded_graphics::prelude::Point;
use epd_waveshare::epd2in7b::Display2in7b;
use epd_waveshare::prelude::TriColor;

use crate::image;
use crate::weather_types::OpenWeatherMapResponse;

use super::Module;



pub struct WeatherModule {
    api_key: String,
    lat: f64,
    lon: f64,
}

impl WeatherModule {
    pub fn new(api_key: String, lat: f64, lon: f64) -> Self {
        Self { api_key, lat, lon }
    }

    fn get_weather(&self) -> OpenWeatherMapResponse {
        ureq::get("https://api.openweathermap.org/data/2.5/onecall")
            .query("appid", &self.api_key)
            .query("lat", &self.lat.to_string())
            .query("lon", &self.lon.to_string())
            .query("exclude", "minutely,hourly,alerts")
            .query("units", "metric")
            .call().expect("call OpenWeather onecall API")
            .into_json().expect("convert response into json")
    }
}

impl Module for WeatherModule {
    fn draw(&self, buffer: &mut Display2in7b, offset_x: i32, offset_y: i32) {
        let weather = self.get_weather();

        // println!("{:?}", weather);
        // Draw weather icon
        image::draw_icon(
            buffer, Point::new(offset_x + 2, offset_y + 2),
            format!("weather/{}.png", weather.current.weather[0].icon), 45,
            TriColor::Chromatic,
        );
    }
}

