use clap::Args;

// TODO: Add support for `swellion fetch "11th Street"`
// Fetch { name: String }
#[derive(Args, Debug)]
pub struct Alias {
    /// Alias name
    name: String,
    /// Latitude
    lat: f32,
    /// Longitude
    long: f32,
}

impl Alias {
    pub fn create(&self) {
        println!("alias create");
    }
}
