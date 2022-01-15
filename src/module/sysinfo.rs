use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    text::{TextStyleBuilder, Baseline, Text},
};
use epd_waveshare::{
    epd2in7b::Display2in7b,
    prelude::TriColor
};
use local_ip_address::local_ip;
use sysinfo::{System, SystemExt, ProcessorExt, ComponentExt};

use crate::image;

use super::Module;


pub struct SystemInfoModule;

impl SystemInfoModule {
    pub fn new() -> Self {
        SystemInfoModule
    }
}

impl Module for SystemInfoModule {
    fn draw(&self, buffer: &mut Display2in7b, offset_x: i32, offset_y: i32) {
        let mut system = System::new_all();
        system.refresh_cpu();
        system.refresh_memory();
        system.refresh_networks();


        // Draw main icon
        image::draw_icon(
            buffer, Point::new(offset_x + 2, offset_y),
            "speed/odometer.jpeg", 45,
            TriColor::Chromatic,
        );

        // Draw texts

        let style = MonoTextStyleBuilder::new()
            .font(&profont::PROFONT_12_POINT)
            .text_color(TriColor::Black)
            .background_color(TriColor::White)
            .build();
        let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

        // CPU Usage
        let cpu = &system.processors()[0];
        cpu.cpu_usage();

        Text::with_text_style(
            &format!("{:.0}%", cpu.cpu_usage()),
            Point::new(offset_x + 82, offset_y + 2), style, text_style
        )
            .draw(buffer)
            .expect("draw cpu usage");

        // CPU Temperature
        let cpu_component = system.components().iter()
            .filter(|c| c.label() == "CPU")
            .next().unwrap();
        Text::with_text_style(
            &format!("{:.2}°C", cpu_component.temperature()),
            Point::new(offset_x + 82, offset_y + 22), style, text_style
        )
            .draw(buffer)
            .expect("draw cpu temperature");

        // Memory usage
        Text::with_text_style(
            &format!("{}/{}MiB", system.used_memory() / 1024, system.total_memory() / 1024),
            Point::new(offset_x + 160, offset_y + 2), style, text_style
        )
            .draw(buffer)
            .expect("draw memory usage");

        // IP-Address
        Text::with_text_style(
            &format!("{}", local_ip().unwrap()),
            Point::new(offset_x + 160, offset_y + 22), style, text_style
        )
            .draw(buffer)
            .expect("draw ip-address");
    }
}
