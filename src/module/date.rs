use chrono::prelude::*;
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    text::{TextStyleBuilder, Baseline, Text},
    prelude::*
};
use epd_waveshare::{
    epd2in7b::Display2in7b,
    prelude::*
};

use super::Module;


pub struct DateModule;

impl DateModule {
    pub fn new() -> Self {
        DateModule
    }
}

impl Default for DateModule {
    fn default() -> Self {
        Self::new()
    }
}


impl Module for DateModule {
    fn draw(&self, buffer: &mut Display2in7b, offset_x: i32, offset_y: i32) {
        // Get local time
        let local = Local::now();

        // Create the base text styles
        // TODO: Global text style?!
        let mut style = MonoTextStyleBuilder::new()
            .font(&profont::PROFONT_24_POINT)
            .text_color(TriColor::Chromatic)
            .background_color(TriColor::White)
            .build();
        let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

        // Draw the day of the week
        let Point { x: w, .. } = Text::with_text_style(&local.format("%A").to_string(), Point::new(offset_x + 2, offset_y + 2), style, text_style)
            .draw(buffer)
            .expect("Draw weekday");

        // Update style with smaller font and black instead of red letters.
        style.font = &profont::PROFONT_18_POINT;
        style.text_color = Some(TriColor::Black);

        // Draw the date
        Text::with_text_style(&local.format("%b %-d, %Y").to_string(), Point::new(w + 7, offset_y + 8), style, text_style)
            .draw(buffer)
            .expect("Draw date");
    }
}
