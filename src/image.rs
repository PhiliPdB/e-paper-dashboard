use embedded_graphics::{
    image::{ImageRaw, Image},
    pixelcolor::raw::RawU8,
    prelude::*,
};
use epd_waveshare::{
    epd2in7b::Display2in7b,
    prelude::*,
};
use image::{
    imageops::{self, FilterType},
    io::Reader as ImageReader,
    EncodableLayout, Pixel,
};


pub fn draw_icon(
    target: &mut Display2in7b, location: Point,
    path: String, size: u32,
    on_color: TriColor,
) {
    let img = ImageReader::open(format!("./assets/{}", path))
        .expect("Opening image")
        .decode().expect("Decoding image")
        .to_luma8();

    let mut img = imageops::resize(&img, size, size, FilterType::Triangle);
    let threshold = imageproc::contrast::otsu_level(&img);
    imageproc::contrast::threshold_mut(&mut img, threshold);

    // Convert black pixels to the given `on_color`.
    for p in img.pixels_mut() {
        if p.channels()[0] == 0x00 {
            p.channels_mut()[0] = RawU8::from(on_color).into_inner();
        }
    }

    // Convert image into one that can be drawn by embedded_graphics.
    let raw_image = ImageRaw::<TriColor>::new(img.as_bytes(), size);
    let embedded_image = Image::new(&raw_image, location);
    embedded_image.draw(target)
        .expect("draw image");
}
