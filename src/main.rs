use structopt::StructOpt;
use exitfailure::{ExitFailure};
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use dotenv::dotenv;

#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let place =  (&args.city, &args.country_code);
    let resp = Forecast::get(place).await?;
    let temp_cel = kelvin_to_celcius(resp.main.temp);
    let wind_speed = miles_per_sec_to_kmh(resp.wind.speed);
    let wind_direction = degrees_to_compass(resp.wind.deg);

    println!("{}:", place.0);    
    println!("Moln: {}", resp.weather.details.description);
    println!("Tempratur: {:.2}°C", temp_cel);
    println!("Fuktighet: {}%", resp.main.humidity);
    println!("Vind hastighet: {:.2}km/h", wind_speed);
    println!("Vind rikting: {}", wind_direction);
    Ok(())

}

fn kelvin_to_celcius(kel: f64) -> f64{
     kel - 273.15
}
fn degrees_to_compass(deg: i32) -> & 'static str {
    match deg {
        00..=21 => return "Nord",
        22..=43 => return "Nord Nordöst",
        44..=45 => return "Nordöst",
        46..=66 => return "Öst Sydöst",
        67..=111 => return "Öst",
        112..=133 => return "Öst Sydöst",
        134..=135 => return "Sydöst",
        136..=156 => return "Syd Sydöst",
        157..=201 => return "Syd",
        202..=223 => return "Syd Sydväst",
        224..=225 => return "Sydväst",
        226..=246 => return "Väst Sydväst",
        247..=291 => return "Väst",
        292..=313 => return "West Nordväst",
        314..=315 => return "Nordväst",
        316..=336 => return "Nord Nordväst",
        337..=360 => return "Nord",
        _ => return "Error med att hämta vindriktingen",
    }
}
fn miles_per_sec_to_kmh(inputspeed: f64) -> f64 {
    inputspeed * 3.6
}

impl Forecast {
    async fn get(place: (&String,&String)) -> Result<Self,ExitFailure>{
        dotenv().ok();
        let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");
        let lang = "sv";
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&lang={}", place.0,place.1, api_key, lang);

        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(resp)
    }
}

