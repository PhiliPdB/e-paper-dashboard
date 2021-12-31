use epd_waveshare::{
    epd2in7b::{Epd2in7b, Display2in7b},
    prelude::*,
};
use linux_embedded_hal::{
    spidev::{SpidevOptions, SpiModeFlags},
    Delay, Spidev,
};
use rppal::gpio::{Gpio, OutputPin, InputPin};


pub struct PiDisplay {
    spi: Spidev,
    epd: Epd2in7b<Spidev, OutputPin, InputPin, OutputPin, OutputPin, Delay>,
    display: Display2in7b,
    delay: Delay,
}

impl PiDisplay {
    pub fn new() -> Self {
        // Setup gpio pins for display
        let gpio = Gpio::new().expect("get gpio");
        let busy = gpio.get(24).expect("get busy pin")
            .into_input();

        let dc = gpio.get(25).expect("get dc pin")
            .into_output();

        let rst = gpio.get(17).expect("get rst pin")
            .into_output();

        let cs = gpio.get(8).expect("get cs pin")
            .into_output_high();

        // Configure SPI
        let mut spi = Spidev::open("/dev/spidev0.0").expect("spidev directory");
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(4_000_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options).expect("spi configuration");

        let mut delay = Delay {};

        // Setup display and display buffer
        let epd = Epd2in7b::new(&mut spi, cs, busy, dc, rst, &mut delay)
            .expect("e-ink initialization error");
        let display_buffer = Display2in7b::default();

        Self { spi, epd, display: display_buffer, delay }
    }

    pub fn display(&mut self) -> &mut Display2in7b {
        &mut self.display
    }

    pub fn clear(&mut self) {
        self.epd.clear_frame(&mut self.spi, &mut self.delay)
            .expect("clear frame");
        self.epd.display_frame(&mut self.spi, &mut self.delay)
            .expect("display frame");
    }

    pub fn draw(&mut self) {
        self.epd.update_color_frame(&mut self.spi, self.display.bw_buffer(), self.display.chromatic_buffer())
            .expect("update color frame");

        self.epd.display_frame(&mut self.spi, &mut self.delay)
            .expect("display frame");
    }

    pub fn sleep(&mut self) {
        self.epd.sleep(&mut self.spi, &mut self.delay)
            .expect("sleep");
    }
}
