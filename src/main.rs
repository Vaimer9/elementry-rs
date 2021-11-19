use serde_json;

struct Parsed_json {
    
}

fn get_data(file: &str) -> Result<String, std::io::Error> {
    let _ = fs::read_to_string(filename)?;

    return _;
}

fn main() -> Result<(), std::io::Error> {
    let data = get_data("data.json")?;

    let json_data: serde_json::Value = serde_json::from_str(data)?;

    println!("Hello, world!");

    Ok(())
}
