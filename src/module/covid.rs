use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{TextStyleBuilder, Baseline, Text};
use epd_waveshare::epd2in7b::Display2in7b;
use epd_waveshare::prelude::TriColor;
use ureq::Request;

use crate::covid_types::CovidInformation;
use crate::image;

use super::Module;


pub struct CovidModule {
    country: String,
}

impl CovidModule {
    pub fn new(country: String) -> Self {
        Self { country }
    }

    fn get_covid_data(&self) -> CovidInformation {
        let mut information: CovidInformation = self.get_request(false)
            .call().expect("Get today's covid information")
            .into_json().expect("Parse covid information response");

        if information.today_cases.is_none() {
            // Today's information is not available, so get yesterday's information
            information = self.get_request(true)
                .call().expect("Get today's covid information")
                .into_json().expect("Parse covid information response");
        }

        information
    }

    fn get_request(&self, yesterday: bool) -> Request {
        let yesterday =
            if yesterday {
                "true"
            } else {
                "false"
            };

        ureq::get(&format!("https://disease.sh/v3/covid-19/countries/{}", self.country))
            .query("allowNull", "true")
            .query("strict", "true")
            .query("yesterday", yesterday)
    }
}

impl Module for CovidModule {
    fn draw(&self, buffer: &mut Display2in7b, offset_x: i32, offset_y: i32) {
        let covid_data = self.get_covid_data();

        // Draw module icon
        image::draw_icon(
            buffer, Point::new(offset_x + 2, offset_y + 2),
            "covid/covid.jpeg", 45,
            TriColor::Chromatic
        );


        // Draw other icons
        // Icon for total stats
        image::draw_icon(
            buffer, Point::new(offset_x + 50, offset_y + 12),
            "covid/total.jpg", 25, TriColor::Black
        );
        // Icon for today stats
        image::draw_icon(
            buffer, Point::new(offset_x + 160, offset_y + 12),
            "covid/today.jpg", 25, TriColor::Black
        );

        // Cases icons
        image::draw_icon(
            buffer, Point::new(offset_x + 82, offset_y + 2),
            "covid/cough.jpeg", 22, TriColor::Black
        );
        image::draw_icon(
            buffer, Point::new(offset_x + 192, offset_y + 2),
            "covid/cough.jpeg", 22, TriColor::Black
        );
        // Deaths icons
        image::draw_icon(
            buffer, Point::new(offset_x + 82, offset_y + 24),
            "covid/skull.jpeg", 22, TriColor::Black
        );
        image::draw_icon(
            buffer, Point::new(offset_x + 192, offset_y + 24),
            "covid/skull.jpeg", 22, TriColor::Black
        );

        // Add texts
        let style = MonoTextStyleBuilder::new()
            .font(&profont::PROFONT_12_POINT)
            .text_color(TriColor::Black)
            .background_color(TriColor::White)
            .build();
        let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

        // Display cases

        Text::with_text_style(
            &format!("{:.3}M", covid_data.cases as f64 / 1e6),
            Point::new(offset_x + 110, offset_y + 7), style, text_style
        )
            .draw(buffer)
            .expect("draw total cases");

        Text::with_text_style(
            &format!("{}", covid_data.today_cases.unwrap()),
            Point::new(offset_x + 218, offset_y + 7), style, text_style
        )
            .draw(buffer)
            .expect("draw today cases");


        // Display deaths

        Text::with_text_style(
            &format!("{}", covid_data.deaths),
            Point::new(offset_x + 110, offset_y + 27), style, text_style
        )
            .draw(buffer)
            .expect("draw total deaths");

        Text::with_text_style(
            &format!("{}", covid_data.today_deaths.unwrap()),
            Point::new(offset_x + 218, offset_y + 27), style, text_style
        )
            .draw(buffer)
            .expect("draw today deaths");

    }
}
