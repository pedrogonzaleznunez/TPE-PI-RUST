#![allow(non_snake_case)]

use std::collections::BTreeMap;
use std::collections::HashMap;

use std::error::Error;
use std::result::Result;

mod constants;
mod parse_csv;

use constants::CITY_CONFIG;
use parse_csv::readTypesCsv;

use parse_csv::readReqCsvQ1;
use parse_csv::readReqCsvQ2;
// use parse_csv::readReqCsvQ3;
// use parse_csv::readReqCsvQ4;
// use parse_csv::readReqCsvQ5;

#[cfg(all(feature = "nyc", feature = "chi"))]
compile_error!("Only one of `nyc` or `chi` features can be enabled");

#[cfg(not(any(feature = "nyc", feature = "chi")))]
compile_error!("`nyc` or `chi` must be selected");

fn main() -> Result<(), Box<dyn Error>> {
    // structures for query 1
    let mut typesByAcronym: HashMap<String, String> = HashMap::new();
    let mut typesByAgencyBySize: BTreeMap<String, BTreeMap<String, i32>> = BTreeMap::new();

    // structures for query 2
    let mut boroughLatLngBySize: BTreeMap<(String, i32, i32), i32> = BTreeMap::new();

    // QUERY 1 - read csv files
    readTypesCsv(
        CITY_CONFIG.typesFilePath,
        &mut typesByAcronym,
        &mut typesByAgencyBySize,
    )?;

    readReqCsvQ1(
        CITY_CONFIG.requestsFilePath,
        &mut typesByAcronym,
        &mut typesByAgencyBySize,
    )?;

    // QUERY 2 - read csv files
    readReqCsvQ2(CITY_CONFIG.requestsFilePath, &mut boroughLatLngBySize)?;

    // print for query1
    // typesByAgencyBySize.iter().for_each(|(infr, b)| {
    //     b.iter()
    //         .for_each(|(agency, v)| println!("{} ({}) - {}", infr, agency, v))
    // });

    // print for query2
    boroughLatLngBySize
        .iter()
        .for_each(|((borough, lat, lng), v)| println!("{};{};{};{}", borough, lat, lng, v));

    Ok(())
}
