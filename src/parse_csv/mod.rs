#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::BTreeMap;
use std::collections::HashMap;

use std::error::Error;
use std::result::Result;

use super::constants::CITY_CONFIG;

enum TypesFields {
    Name = 0,
    Acronym = 1,
}

pub fn readTypesCsv(
    filePath: &str,
    typesByAcronym: &mut HashMap<String, String>,
    typesByAgencyBySize: &mut BTreeMap<String, BTreeMap<String, i32>>,
) -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let path = root.join(filePath);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            continue;
        }

        let line = line?;

        let fields: Vec<&str> = line.split(';').collect();

        // add key and value to typesByAcronym map
        typesByAcronym.insert(
            fields[TypesFields::Acronym as usize].to_string(),
            fields[TypesFields::Name as usize].to_string(),
        );
        // add just key to typesByAgencyBySize map
        typesByAgencyBySize.insert(
            fields[TypesFields::Acronym as usize].to_string(),
            BTreeMap::new(),
        );
    }

    Ok(())
}

pub fn readReqCsv(
    filePath: &str,
    typesByAcronym: &mut HashMap<String, String>,
    typesByAgencyBySize: &mut BTreeMap<String, BTreeMap<String, i32>>,
) -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let path = root.join(filePath);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            continue;
        }

        let line = line?;

        let fields: Vec<&str> = line.split(';').collect();

        let agencyName: &str = fields[CITY_CONFIG.requestCSVFields.AgencyName as usize];
        let acronym: &str = fields[CITY_CONFIG.requestCSVFields.Acronym as usize];

        let typeName: &str = typesByAcronym
            .get(acronym)
            .expect("Type name must be defined");

        typesByAgencyBySize
            .entry(typeName.to_string())
            .or_insert_with(BTreeMap::new)
            .entry(agencyName.to_string())
            .and_modify(|count: &mut i32| *count += 1)
            .or_insert(1);
    }

    Ok(())
}
