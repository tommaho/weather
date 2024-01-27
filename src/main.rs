use toml::Value; //for api key
use std::env;
use serde::Deserialize;

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
pub struct City {
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


// 

fn main() {

    let api_key = get_api_key();
    let zip_code = parse_args();

    //debug
    //println!("Zip code is: {}", &zip_code);
    //println!("API key is {}", &api_key);

    match fetch_coords(&api_key, &zip_code) {
        Ok(coords) => {
            
            match fetch_weather(&api_key, &coords) {

                Ok(weather_data) => {
                    display_current_weather_data(weather_data);
                },
                Err(weather_error) => {
                    println!("Error fetching weather data: {}", weather_error);
                },
            };

            println!("going to get forecast next")

            
        },
        Err(error) => println!("Error fetching weather data: {}", error),
    };

    
}

fn parse_args() -> String{

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        args[1].clone()
    } else {
        println!("Defaulting to zip code 17701.");
        "17701".to_string()
    }

}



fn fetch_coords(api_key: &str, zip_code: &str) -> Result<Coords, reqwest::Error> {
    //debug
    //println!("The zip we'll look for is {} using {}", zip_code, api_key);

    let url = format!(
        "http://api.openweathermap.org/geo/1.0/zip?zip={},US&appid={}",
        zip_code, api_key
    );

    let response = reqwest::blocking::get(&url)?.json::<Coords>()?;

    Ok(response)

}

fn fetch_weather(api_key: &str, coords: &Coords)-> Result<CurrentWeatherData, reqwest::Error>{
    //debug
    //println!("We'll look up {} {} using {}", coords.lat, coords.lon, api_key);
    
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=imperial",
        coords.lat, coords.lon, api_key
    );

    let response = reqwest::blocking::get(&url)?.json::<CurrentWeatherData>()?;

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

//debug
// fn display_coords(coords: &Coords) {
//     println!("Lat {}:", coords.lat);
//     println!("Lon {}", coords.lon);
// }

fn display_current_weather_data(weather_data: CurrentWeatherData) {
    println!("\nCurrent weather conditions for {}:", weather_data.name);
    println!("Temperature: {} °F", weather_data.main.temp);
    println!("Feels like: {} °F", weather_data.main.feels_like);
    println!("Description: {}\n", weather_data.weather[0].description);
}