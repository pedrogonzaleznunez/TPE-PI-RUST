#![allow(non_snake_case)]
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;
// use std::collections::BTreeMap;

use std::error::Error;
use std::result::Result;

// -> Result<()>
// means it may return an empty tuple () if everything's alright, or an error if something went wrong

fn main() -> Result<(), Box<dyn Error>> {
    let mut typesByAcronym: HashMap<String, String> = HashMap::new();
    // let typesByAgencyBySize: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();

    readTypesCsvNyc(&mut typesByAcronym)?;

    typesByAcronym.iter().for_each(|f| println!("{:?}", f));

    Ok(())
}

fn readTypesCsvNyc(typesByAcronym: &mut HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let path = root.join("resources/Dataset Alumnos/typesNYC.csv");

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            continue;
        }

        let line = line?;

        let fields: Vec<&str> = line.split(';').collect();
        typesByAcronym.insert(fields[0].to_string(), fields[1].to_string()); // to_string turns them into Strings (owned), instead of &str (borrowed, same lifetime as `line`)
    }

    Ok(())
}
