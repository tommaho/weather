use toml::Value; //for api key
use std::env;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Coords {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Deserialize)]
struct WeatherData {
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

fn main() {

    let api_key = get_api_key();
    let zip_code = parse_args();

    println!("Zip code is: {}", &zip_code);
    println!("API key is {}", &api_key);

    match fetch_coords(&api_key, &zip_code) {
        Ok(coords) => {
            
            //display_coords(&coords)//;

            match fetch_weather(&api_key, &coords) {

                Ok(weather_data) => {
                    display_weather_data(weather_data);
                },
                Err(weather_error) => {
                    println!("Error fetching weather data: {}", weather_error);
                },
            }

            
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

    println!("The zip we'll look for is {} using {}", zip_code, api_key);

    let url = format!(
        "http://api.openweathermap.org/geo/1.0/zip?zip={},US&appid={}",
        zip_code, api_key
    );

    let response = reqwest::blocking::get(&url)?.json::<Coords>()?;

    Ok(response)

}

fn fetch_weather(api_key: &str, coords: &Coords)-> Result<WeatherData, reqwest::Error>{

    println!("We'll look up {} {} using {}", coords.lat, coords.lon, api_key);

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        coords.lat, coords.lon, api_key
    );

    let response = reqwest::blocking::get(&url)?.json::<WeatherData>()?;

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

fn display_weather_data(weather_data: WeatherData) {
    println!("Current weather conditions for {}:", weather_data.name);
    println!("Temperature: {} °C", weather_data.main.temp);
    println!("Feels like: {} °C", weather_data.main.feels_like);
    println!("Description: {}", weather_data.weather[0].description);
}