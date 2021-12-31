use embedded_graphics::{mono_font::MonoTextStyleBuilder, text::{TextStyleBuilder, Baseline, Text}, prelude::*};
use epd_waveshare::{
    graphics::TriDisplay,
    prelude::*, epd2in7b::Display2in7b
};

use e_paper_dashboard::display::PiDisplay;


fn main() {
    println!("Initializing display");
    let mut display = PiDisplay::new();
    println!("Clearing display");
    display.clear();

    println!("Setting buffer contents");
    display.display().clear_buffer(TriColor::White);
    display.display().set_rotation(DisplayRotation::Rotate270);
    draw_text(display.display(), "Hello World!", 5, 50, TriColor::Black);
    draw_text(display.display(), "Hello World!", 5, 100, TriColor::Chromatic);


    println!("Update screen");
    display.draw();

    println!("Sleep");
    display.sleep();
}


fn draw_text(display: &mut Display2in7b, text: &str, x: i32, y: i32, color: TriColor) {
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_6X10)
        .text_color(color)
        .background_color(TriColor::White)
        .build();

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
}
