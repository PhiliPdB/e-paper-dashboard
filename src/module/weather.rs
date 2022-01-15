use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{TextStyleBuilder, Baseline, Text};
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

        // Draw weather icon
        image::draw_icon(
            buffer, Point::new(offset_x + 2, offset_y + 2),
            &format!("weather/{}.png", weather.current.weather[0].icon), 45,
            TriColor::Chromatic,
        );

        // Main weather information
        let mut style = MonoTextStyleBuilder::new()
            .font(&profont::PROFONT_14_POINT)
            .text_color(TriColor::Chromatic)
            .background_color(TriColor::White)
            .build();
        let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

        Text::with_text_style(
            &format!("{:.0}°C", weather.current.temp.round()),
            Point::new(offset_x + 50, offset_y + 8), style, text_style
        )
            .draw(buffer)
            .expect("draw temperature");

        let line_height = style.font.character_size.height as i32;
        style.font = &profont::PROFONT_12_POINT;
        Text::with_text_style(
            &weather.current.weather[0].main,
            Point::new(offset_x + 50, offset_y + 8 + line_height), style, text_style
        )
            .draw(buffer)
            .expect("draw weather description");

        // Secondary information
        let style = MonoTextStyleBuilder::from(&style)
            .font(&profont::PROFONT_10_POINT)
            .text_color(TriColor::Black)
            .build();

        let line_height = style.font.character_size.height as i32;
        Text::with_text_style(
            &format!("Feels like {:.0}°", weather.current.feels_like.round()),
            Point::new(offset_x + 102, offset_y + 11), style, text_style
        )
            .draw(buffer)
            .expect("draw feels like");
        Text::with_text_style(
            &format!("Humidity {}%", weather.current.humidity),
            Point::new(offset_x + 102, offset_y + 11 + line_height), style, text_style
        )
            .draw(buffer)
            .expect("draw humidity");

        Text::with_text_style(
            &format!("Day {:.0}°", weather.daily[0].temp.day.round()),
            Point::new(offset_x + 200, offset_y + 11), style, text_style
        )
            .draw(buffer)
            .expect("draw day temp");
        Text::with_text_style(
            &format!("Night {:.0}°", weather.daily[0].temp.night.round()),
            Point::new(offset_x + 200, offset_y + 11 + line_height), style, text_style
        )
            .draw(buffer)
            .expect("draw night temp");

    }
}
