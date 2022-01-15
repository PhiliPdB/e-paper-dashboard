use dotenv::dotenv;
use epd_waveshare::{
    graphics::TriDisplay,
    prelude::*,
};
use structopt::StructOpt;

use e_paper_dashboard::{
    config::Config,
    dashboard::Dashboard,
    module::{Module, date::DateModule, weather::WeatherModule, covid::CovidModule, sysinfo::SystemInfoModule}
};


fn main() {
    // Load .env environment
    dotenv().ok();
    // Get configuration
    let config: Config = Config::from_args();

    println!("Initializing display");
    let mut dashboard = Dashboard::new();
    println!("Clearing display");
    dashboard.clear();

    println!("Setting buffer contents");
    dashboard.display().clear_buffer(TriColor::White);
    dashboard.display().set_rotation(DisplayRotation::Rotate270);



    // TODO: Move this inside the dashboard struct
    let date = DateModule::new();
    date.draw(dashboard.display(), 0, 0);

    let weather = WeatherModule::new(
        config.open_weather_map_api_key,
        config.latitude, config.longitude
    );
    weather.draw(dashboard.display(), 0, 32);

    let covid = CovidModule::new(config.country);
    covid.draw(dashboard.display(), 0, 80);

    let sys_info = SystemInfoModule::new();
    sys_info.draw(dashboard.display(), 0, 128);

    // e_paper_dashboard::image::draw_icon(
    //     dashboard.display(), Point::new(5, 5),
    //     String::from("weather/cloud.png"), 45,
    //     TriColor::Chromatic,
    // );


    println!("Update screen");
    dashboard.draw();

    println!("Sleep");
    dashboard.sleep();
}
