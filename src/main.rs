#![allow(non_snake_case)]
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use std::collections::HashMap;
use std::collections::BTreeMap;

use std::option;

// -> Result<()>
// significa que puede devolver una tupla vacia () si todo sale bien, o un error si algo falla

fn main() -> Result<()> {

    let typesByAcronym: HashMap<String, String> = HashMap::new();
    let typesByAgencyBySize: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    // Abrir el archivo
    let root = std::env::current_dir()?;
    let path = root.join("resources/Dataset Alumnos/typesCHI.csv");

    println!("{}", path.display());
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line?; // obtengo la linea como un Result<String>
        let fields: Vec<&str> = line.split(',').collect();

        if i == 0 {
            println!("{:?}", fields);
            println!("{}", '\n');
        } else {
            println!("{} {:?}", i, fields);
        }
    }

    Ok(())
}

fn readTypesCsvNyc(typesByAcronym: &HashMap<String, String> ) -> Option<i32> {

    let root = std::env::current_dir().ok()?;
    let path = root.join("resources/Dataset Alumnos/typesNYC.csv");

    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        
        if i == 0 {
            continue;
        }

        let line = line.ok()?;

        let fields: Vec<&str> = line.split(';').collect();

        println!();
    }

    Some(42)
} 
