mod env;
use reqwest::{blocking::get, header::EXPECT};
use serde_json::{Value,from_str};
use std::io;

fn main(){
    println!("Herzlich Willkommen! Hier kannst du dir das Wetter anzeigen lassen!"); 

    let mut city = String::new();

    println!("Bitte gib deine Stadt ein: ");

    io::stdin()
        .read_line(&mut city)  
        .expect("Fehler beim Lesen der Eingabe");


    let api_key = env::API_KEY;
    let url: String = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",city,api_key);
    let response = get(url).unwrap();
    let response_text: String = response.text().unwrap();
    let json: Value = from_str(&response_text).expect("JSON was not well-formated");

    let temp_kelvin: f64 =json["main"]["temp"].as_f64().unwrap();
    let temp_celsius: f64 = temp_kelvin - 273.15;


    println!("Hier ist das Wetter in {}:",city);
    println!("Aktuell {} Grad Celsius",temp_celsius.round());    

    if temp_celsius >= 20.0 {
        println!("Ziemlich warm!");
    }else {
        print!("Ganz schön kalt!")
    }

    println!("\n\n");

}