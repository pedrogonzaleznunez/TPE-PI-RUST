#![allow(non_snake_case)]

use std::collections::BTreeMap;
use std::collections::HashMap;

use std::error::Error;
use std::result::Result;

mod constants;
mod parse_csv;

use constants::CITY_CONFIG;
use parse_csv::readReqCsv;
use parse_csv::readTypesCsv;

#[cfg(all(feature = "nyc", feature = "chi"))]
compile_error!("Only one of `nyc` or `chi` features can be enabled");

#[cfg(not(any(feature = "nyc", feature = "chi")))]
compile_error!("`nyc` or `chi` must be selected");

fn main() -> Result<(), Box<dyn Error>> {
    let mut typesByAcronym: HashMap<String, String> = HashMap::new();
    let mut typesByAgencyBySize: BTreeMap<String, BTreeMap<String, i32>> = BTreeMap::new();

    readTypesCsv(
        CITY_CONFIG.typesFilePath,
        &mut typesByAcronym,
        &mut typesByAgencyBySize,
    )?;

    readReqCsv(
        CITY_CONFIG.requestsFilePath,
        &mut typesByAcronym,
        &mut typesByAgencyBySize,
    )?;

    typesByAgencyBySize.iter().for_each(|(infr, b)| {
        b.iter()
            .for_each(|(agency, v)| println!("{} ({}) - {}", infr, agency, v))
    });

    Ok(())
}
