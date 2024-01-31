mod env;
use reqwest::blocking::get;
use serde_json::{Value,from_str};

fn main(){
    println!("Herzlich Willkommen! Hier kannst du dir das Wetter anzeigen lassen!"); 
    let city: &str = "Berlin";
    let api_key = env::API_KEY;
    let url: String = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",city,api_key);
    let response = get(url).unwrap();
    let response_text: String = response.text().unwrap();
    let json: Value = from_str(&response_text).expect("JSON was not well-formated");

    println!("Hier ist das Wetter in {}:",city);
    println!("{}",json["main"]["temp"]);    

    println!("\n\n");

}