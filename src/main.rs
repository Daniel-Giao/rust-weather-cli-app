use colored::*;
use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}
