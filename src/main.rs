#![allow(non_snake_case)]

use std::collections::BTreeMap;
use std::collections::HashMap;

use std::error::Error;
use std::result::Result;

mod constants;
mod ctable;
mod parse_csv;

use constants::CITY_CONFIG;
use parse_csv::readTypesCsv;

use parse_csv::readReqCsvQ1;
use parse_csv::readReqCsvQ2;

use crate::ctable::HTMLTable;
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

    let mut table = HTMLTable::new("output_query1.html", vec!["type", "agency", "requests"])?;

    for (infr, agencies) in &typesByAgencyBySize {
        for (agency, count) in agencies {
            table.add_row(vec![
                infr.as_str(),
                agency.as_str(),
                count.to_string().as_str(),
            ])?;
        }
    }

    table.close()?;

    table = HTMLTable::new(
        "output_query2.html",
        vec!["borough", "latitude", "longitude", "requests"],
    )?;

    for ((borough, lat, long), count) in &boroughLatLngBySize {
        table.add_row(vec![
            borough.as_str(),
            lat.to_string().as_str(),
            long.to_string().as_str(),
            count.to_string().as_str(),
        ])?;
    }

    table.close()?;

    Ok(())
}
