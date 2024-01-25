use toml::Value; //for api key
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let zip_code = parse_args(&args);

    println!("Zip code is: {}", zip_code);


    println!("Hello.");

    let api_key = get_api_key();

    println!("API key is {}", api_key);


}

fn parse_args(args: &[String]) -> &str{

    if args.len() >= 2 {
        &args[1]
    } else {
        println!("No zip provided, defaulting to 17701.");
        "17701"
    }

    // let zip_code = &args[1];
    // zip_code
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