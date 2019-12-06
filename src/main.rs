use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    println!("{}", serde_json::to_string(&args)?);
    Ok(())
}
