use clap::Args;
use std::{error::Error, fmt, collections::HashMap};

#[derive(Debug)]
pub struct FetchError(String);

impl Error for FetchError {}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

#[derive(Args, Debug)]
pub struct Fetch {
    /// Latitude
    lat: f32,
    /// Longitude
    long: f32,
}

enum BuoyKeys {
    Year = 0,
    Month = 1,
    Day = 2,
    Hour = 3,
    Minute = 4,
    WindDir = 5,
    WindSpeed = 6,
    WingGust = 7,
    WaveHeight = 8,
    DominantPeriod = 9,
    AveragePeriod = 10,
    WaveDirection = 11,
    Presssure = 12,
    AirTemp = 13,
    WaterTemp = 14,
    DewPoint = 15,
    Visibility = 16,
    PressureTendency = 17,
    Tide = 18,
}

impl Fetch {
    pub async fn fetch(&self) -> Result<(), Box<dyn Error>> {
        let resp = reqwest::get("https://www.ndbc.noaa.gov/data/realtime2/46266.txt")
            .await?
            .text()
            .await?;
        let lines = resp.split("\n").collect::<Vec<&str>>();

        let data = match lines.get(2) {
            Some(line) => {
                println!("{}", line);
                line
            }
            None => return Err(Box::new(FetchError("Invalid data from API".into()))),
        };

        let data_points = data
            .split(" ")
            .into_iter()
            .filter(|&s| !s.is_empty())
            .collect::<Vec<&str>>();

        println!("{:?}", data_points);

        if data_points.len() != 19 {
            println!("{:?}", data_points.get(BuoyKeys::Year as i32));
        }

        Ok(())
    }
}
