use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoscreenConfig {
    pub openweathermap_key: String,

    pub digitransit_key: String,
    pub digitransit_query_stops: String,
    pub digitransit_query_bike_stations: String,

    pub wodconnect_gym_id: String,
    pub wodconnect_cookie: String,

    pub work_directory: String,
    pub static_file_directory: String,
    pub listen_address: String
}

impl Default for InfoscreenConfig {
    fn default() -> Self {
        InfoscreenConfig {
            openweathermap_key: String::from(""),

            digitransit_key: String::from(""),
            digitransit_query_stops: String::from(""),
            digitransit_query_bike_stations: String::from(""),

            wodconnect_gym_id: String::from(""),
            wodconnect_cookie: String::from(""),

            work_directory: String::from(""),
            static_file_directory: String::from(""),
            listen_address: String::from("")
        }
    }
}
