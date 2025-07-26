#![allow(non_snake_case)]

use std::collections::BTreeMap;
use std::collections::HashMap;

use std::error::Error;
use std::result::Result;

mod constants;
mod ctable;
mod parse_csv;

use std::ffi::CString;

use constants::CITY_CONFIG;
use parse_csv::readTypesCsv;

use parse_csv::readReqCsvQ1;
use parse_csv::readReqCsvQ2;

use crate::ctable::addHTMLRow;
use crate::ctable::closeHTMLTable;
use crate::ctable::newTable;
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

    unsafe {
        let mut table = newTable(
            CString::new("output_query1.html").unwrap().as_ptr(),
            3,
            CString::new("type").unwrap().as_ptr(),
            CString::new("agency").unwrap().as_ptr(),
            CString::new("requests").unwrap().as_ptr(),
        );

        for (infr, agencies) in &typesByAgencyBySize {
            for (agency, count) in agencies {
                addHTMLRow(
                    table,
                    CString::new(infr.as_str())
                        .expect("Failed to create CString")
                        .as_ptr(),
                    CString::new(agency.as_str())
                        .expect("Failed to create CString")
                        .as_ptr(),
                    CString::new(count.to_string())
                        .expect("Failed to create CString")
                        .as_ptr(),
                );
            }
        }

        closeHTMLTable(table);

        table = newTable(
            CString::new("output_query2.html").unwrap().as_ptr(),
            4,
            CString::new("borough").unwrap().as_ptr(),
            CString::new("latitude").unwrap().as_ptr(),
            CString::new("longitude").unwrap().as_ptr(),
            CString::new("requests").unwrap().as_ptr(),
        );

        for ((borough, lat, long), count) in &boroughLatLngBySize {
            addHTMLRow(
                table,
                CString::new(borough.as_str())
                    .expect("Failed to create CString")
                    .as_ptr(),
                CString::new(lat.to_string())
                    .expect("Failed to create CString")
                    .as_ptr(),
                CString::new(long.to_string())
                    .expect("Failed to create CString")
                    .as_ptr(),
                CString::new(count.to_string())
                    .expect("Failed to create CString")
                    .as_ptr(),
            );
        }

        closeHTMLTable(table);
    }

    Ok(())
}
