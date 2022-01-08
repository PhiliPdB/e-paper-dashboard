use epd_waveshare::epd2in7b::Display2in7b;

pub mod date;
pub mod weather;


pub trait Module {
    fn draw(&self, buffer: &mut Display2in7b, offset_x: i32, offset_y: i32);
}
