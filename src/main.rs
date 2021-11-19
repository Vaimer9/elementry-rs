extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fs;

struct Total_data {
    elements: Vec<Info>
}

#[derive(Serialize, Deserialize)]
struct Info {
    name: String,
    appearance: String,
    atomic_mass: f32,
    boil: f32,
    category: String,
    density: f32,
    discovered_by: String,
    melt: f32,
    molar_heat: f32,
    named_by: String,
    number: String,
    period: String,
    phase: String,
    source: String,
    spectral_img: String,
    summary: String,
    symbol: String,
    xpos: f32,
    ypos: f32,
    shells: Vec<isize>,
    electron_configuration: String,
    electron_configuration_semantic: String,
    electron_affinity: f32,
    electronegativity_pauling: f32,
    ionization_energies: Vec<f64>,
    cpk_hex: String
}

fn get_data(file: &str) -> Result<Info, std::io::Error> {
    let x = fs::read_to_string(file)?;
    let data: Info = serde_json::from_str(&x)?;

    return Ok(data);
}

fn main() -> Result<(), std::io::Error> {
    let data = get_data("data.json")?;


    println!("{}", data.name);

    Ok(())
}
