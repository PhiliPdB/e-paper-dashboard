use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CovidInformation {
    pub updated: u64,
    pub country: String,
    pub country_info: CountryInformation,
    pub continent: String,

    pub cases: u64,
    pub today_cases: Option<u64>,
    pub deaths: u64,
    pub today_deaths: Option<u64>,
    pub recovered: u64,
    pub today_recovered: Option<u64>,
    pub tests: u64,

    pub active: u64,
    pub critical: u64,

    pub population: u64,
    pub cases_per_one_million: f64,
    pub deaths_per_one_million: f64,
    pub recovered_per_one_million: f64,
    pub active_per_one_million: f64,
    pub critical_per_one_million: f64,

    pub one_case_per_people: u32,
    pub one_death_per_people: u32,
    pub one_test_per_people: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryInformation {
    #[serde(rename = "_id")]
    pub id: u32,
    pub iso2: String,
    pub iso3: String,
    pub lat: f64,
    pub long: f64,
    pub flag: String,
}
