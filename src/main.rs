use toml::Value; //for api key
use std::env;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Coords {
    lat: f64,
    lon: f64,
}

fn main() {

    let api_key = get_api_key();
    let zip_code = parse_args();

    println!("Zip code is: {}", &zip_code);
    println!("API key is {}", &api_key);

    //let _ = get_lat_lon(&zip_code, &api_key);

    match fetch_coords(&api_key, &zip_code) {
        Ok(coords) => display_coords(coords),
        Err(error) => println!("Error fetching weather data: {}", error),
    }

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


fn get_api_key() -> String {

    let config_content = std::fs::read_to_string("Config.toml")
        .expect("Unable to read config.");
    let config: Value = toml::from_str(&config_content)
        .expect("Error parsing config.");
    let api_key = config["weather_api_key"].as_str()
        .expect("API_KEY not found in config file");

    api_key.to_string()
}


fn display_coords(coords: Coords) {
    println!("Lat {}:", coords.lat);
    println!("Lon {}", coords.lon);
}


// fn get_forecast(lat_lon: (f64, f64)){
//     println!("Lat Lon is {}, {}", lat_lon.0, lat_lon.1);
//     ()
// }