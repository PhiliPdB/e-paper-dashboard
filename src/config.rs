use structopt::StructOpt;


#[derive(Debug, Clone, StructOpt)]
#[structopt()]
pub struct Config {
    #[structopt(long = "api_key", env, hide_env_values = true)]
    /// Api key for calling the openweathermap API.
    pub open_weather_map_api_key: String,

    #[structopt(long = "lat", env = "LAT")]
    /// Latitude of the current location
    pub latitude: f64,

    #[structopt(long = "lon", env = "LON")]
    /// Longitude of the current location
    pub longitude: f64,

    #[structopt(long, env)]
    /// Country to get COVID-19 information for.
    pub country: String,
}
