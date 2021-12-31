use epd_waveshare::{
    graphics::TriDisplay,
    prelude::*
};

use e_paper_dashboard::{dashboard::Dashboard, module::{Module, date::DateModule}};


fn main() {
    println!("Initializing display");
    let mut display = Dashboard::new();
    println!("Clearing display");
    display.clear();

    println!("Setting buffer contents");
    display.display().clear_buffer(TriColor::White);
    display.display().set_rotation(DisplayRotation::Rotate270);

    // TODO: Move this inside the dashboard struct
    let date = DateModule::new();
    date.draw(display.display(), 0, 0);


    println!("Update screen");
    display.draw();

    println!("Sleep");
    display.sleep();
}
