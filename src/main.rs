use toml::Value; //for api key
use std::env;

fn main() {

    let api_key = get_api_key();
    let zip_code = parse_args();

    println!("Zip code is: {}", &zip_code);
    println!("API key is {}", &api_key);

    let (lat, lon) = get_lat_lon(&zip_code);

    println!("lat lon is {} {}", lat, lon);

    let _ = get_forecast((lat, lon));

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

fn get_lat_lon(zip_code: &str) -> (f64, f64) {
    println!("The zip we'll look for is {}", zip_code);
    (1.0, 1.0)
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

fn get_forecast(lat_lon: (f64, f64)){
    println!("Lat Lon is {}, {}", lat_lon.0, lat_lon.1);
    ()
}