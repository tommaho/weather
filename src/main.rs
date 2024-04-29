///SP24 CIT368 Assignment 1
///Get 3 day weather forecast from an api

use toml::Value; //for api key
use std::env;
use serde::Deserialize;
use serde_json::json;
use chrono::{NaiveDate, Datelike, Local, DateTime};
use jsonschema::is_valid;

use std::fs::OpenOptions;
use std::io::Write;
use regex::Regex;

#[derive(Debug, Deserialize)]
struct Coords {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Deserialize)]
struct CurrentWeatherData {
    name: String,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f32,
    feels_like: f32,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}
//forecast

#[derive(Debug, Deserialize)]
pub struct WeatherForecast {
    pub cod: String,
    pub message: f64,
    pub cnt: u32,
    pub list: Vec<ForecastItem>,
    pub city: ForecastCity,
}

#[derive(Debug, Deserialize)]
pub struct ForecastItem {
    pub dt: i64,
    pub main: ForecastMain,
    pub weather: Vec<ForecastWeather>,
    pub clouds: ForecastClouds,
    pub wind: ForecastWind,
    pub sys: ForecastSys,
    pub dt_txt: String,
}

#[derive(Debug, Deserialize)]
pub struct ForecastMain {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub sea_level: u32,
    pub grnd_level: u32,
    pub humidity: u32,
    pub temp_kf: f64,
}

#[derive(Debug, Deserialize)]
pub struct ForecastWeather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct ForecastClouds {
    pub all: u32,
}

#[derive(Debug, Deserialize)]
pub struct ForecastWind {
    pub speed: f64,
    pub deg: u32,
}

#[derive(Debug, Deserialize)]
pub struct ForecastSys {
    pub pod: String,
}

#[derive(Debug, Deserialize)]
pub struct ForecastCity {
    pub id: u32,
    pub name: String,
    pub coord: Coord,
    pub country: String,
    pub population: u32,
    pub timezone: i32,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Debug, Deserialize)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}


fn main() {

    log("Startup.");

    let api_key = get_api_key();
    let zip_code = parse_args(env::args().collect());

    match fetch_coords(&api_key, &zip_code) {
        Ok(coords) => {
            
            match fetch_weather(&api_key, &coords) {

                Ok(weather_data) => {
                    display_current_weather_data(weather_data);
                },
                Err(weather_error) => {
                    let error_message = format!("Error fetching weather data: {}", weather_error);
                    log(&error_message);
                    println!("{}", error_message);
                },
            };

            match fetch_forecast(&api_key, &coords) {

                Ok(weather_forecast) => {
                    display_forecast_data(weather_forecast);
                },
                Err(forecast_error) => {
                    let error_message = format!("Error fetching forecast data: {}", forecast_error);
                    log(&error_message);
                    println!("{}", error_message);
                },
            };      
        },
        Err(error) => println!("Error fetching weather data: {}", error),
    };

    log("Shutdown.");   
    
}

fn parse_args(args: Vec<String>) -> String{
    
    if args.len() == 2 {
        let zip = args[1].clone();

        let re = Regex::new(r"^\d{5}$").unwrap();

        if re.is_match(&zip) {
            return zip;
        } 
    }
        let message = "Missing or invalid zip, defaulted to 17701.";
        log(&message);   

        println!("\n** {} **", message);
        "17701".to_string()

}



fn fetch_coords(api_key: &str, zip_code: &str) -> Result<Coords, Box<dyn std::error::Error>> { //} reqwest::Error> {

    let url = format!(
        "https://api.openweathermap.org/geo/1.0/zip?zip={},US&appid={}",
        zip_code, api_key
    );

    let raw_response = reqwest::blocking::get(&url)?.json()?;


    //TODO move this into a config file
    let expected_schema = json!({
        "zip": "90210",
        "name": "Beverly Hills",
        "lat": 34.0901,
        "lon": -118.4065,
        "country": "US",
    });

    let schema = json!(expected_schema);
    let instance = raw_response; 
    
    if !is_valid(&schema, &instance) {
        return Err("Invalid response schema".into());
    }

    let coords = Coords {
        lat: instance["lat"].as_f64().unwrap_or_default(),
        lon: instance["lon"].as_f64().unwrap_or_default(),
    };

    Ok(coords)

}

fn fetch_weather(api_key: &str, coords: &Coords)-> Result<CurrentWeatherData, reqwest::Error>{
    
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=imperial",
        coords.lat, coords.lon, api_key
    );

    let response = reqwest::blocking::get(&url)?.json::<CurrentWeatherData>()?;
    
    log("Current weather data retrieved.");  
    
    //schema validation here - need schema template

    Ok(response)
}

fn fetch_forecast(api_key: &str, coords: &Coords)-> Result<WeatherForecast, reqwest::Error>{

    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}&units=imperial",
        coords.lat, coords.lon, api_key
    );

    let response: WeatherForecast = reqwest::blocking::get(&url)?.json::<WeatherForecast>()?;

    log("Forecast data retrieved.");  

    //schema validation here - need schema template

    Ok(response)
}

fn get_api_key() -> String {

    let config_content = std::fs::read_to_string("Config.toml")
        .expect("Unable to read config.");

    let config: Value = toml::from_str(&config_content)
        .expect("Error parsing config.");

    let api_key = config["weather_api_key"].as_str()
        .expect("API_KEY not found in config file");

    api_key.to_string()
}

fn display_current_weather_data(weather_data: CurrentWeatherData) {
    println!("\nCurrent weather conditions for {}:\n", weather_data.name);
    println!("Temperature: \t{} °F", weather_data.main.temp);
    println!("Feels like: \t{} °F", weather_data.main.feels_like);
    println!("Description: \t{}\n", weather_data.weather[0].description);
}

fn display_forecast_data(weather_forecast: WeatherForecast) {

    let current_time = chrono::offset::Local::now();
    let cur_dow =  current_time.date_naive().weekday();
    let mut day_counter = 0;
    let mut record_counter = 0;

    println!("The 3 day forecast:\n");

    for entry in weather_forecast.list {

        let (date, date_rem) = NaiveDate::parse_and_remainder(
            &entry.dt_txt, "%Y-%m-%d").unwrap();
    
        let dow = date.weekday();

        let dow_str = if dow == cur_dow {
            "Today".to_string()
        } else {
            dow.to_string()
        };

        if record_counter %8 == 0 && dow_str != "Today" {
            println!(" ");
            day_counter += 1;
        }
        if day_counter <= 3 { //only 3 day forecast

            println!("{} \t{} \t {:.2}°F {}\t{}"
            , dow_str
            , date_rem //entry.dt_txt
            , entry.main.temp
            , get_weather_symbol(&entry.weather[0].main)
            , entry.weather[0].description);

            if dow_str != "Today" {
                record_counter += 1;
            }

        } else {
            break;
        }

    }
    
}

fn get_weather_symbol(weather: &str) -> &'static str {
    match weather {
        "Clear" => "🌞",
        "Clouds" => "🌥️",
        "Rain" => "🌧️",
        "Snow" => "⛄️",
        _ => "🤷🏼‍♂️", // Default symbol for unknown weather
    }
}


fn log(message: &str) {

    let current_time: DateTime<Local> = Local::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
    let log_message = format!("{}: {}\n", formatted_time, message);


    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt").expect("Logger error.");

        let metadata = file.metadata().expect("Logger error.");
        let file_size = metadata.len();
    
        const MAX_FILE_SIZE: u64 = 20 * 1024;
        if file_size > MAX_FILE_SIZE {
            file.set_len(0).expect("Logger error.");
            log("Log truncated.");
        }
        
    file.write_all(log_message.as_bytes()).expect("Logger error.");
    
    //Might not do this automatically on all platforms?
    //file.write_all(b"\n").expect("Logger error.");

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_zip() {
        let args = vec!["placeholder".to_string(), "12345".to_string()];
        assert_eq!(parse_args(args), "12345".to_string());
    }

    #[test]
    fn test_missing_zip() {
        let args = vec!["placeholder".to_string()];
        assert_eq!(parse_args(args), "17701".to_string());
    }

    #[test]
    fn test_invalid_zip() {
        let args = vec!["placeholder".to_string(), "1".to_string()];
        //a zip that isn't 5 numerics will default to 17701
        assert_eq!(parse_args(args), "17701".to_string());
    }

    #[test]
    fn test_fetch_coords() {
        let api_key = get_api_key(); // Replace with your actual API key
        let zip_code = "17701";

        // fetch_coords will return lat / lon from the weather api
        // {
        //     "zip": "17701",
        //     "name": "Williamsport",
        //     "lat": 41.24,
        //     "lon": -77.02,
        //     "country": "US"
        // }

    
        let result = fetch_coords(&api_key, zip_code);

        match result {
            Ok(coords) => assert_eq!(coords.lat, 41.2412),
            Err(_) => panic!("Test failed: Invalid response or error occurred."),
        }
    }
    #[test]
    fn test_fetch_weather() {
        let api_key = get_api_key(); // Replace with your actual API key
        let zip_code = "17701";
    
        let coords = fetch_coords(&api_key, zip_code).expect("err");

        let result = fetch_forecast(&api_key, &coords);

        assert!(result.is_ok());
    }

}