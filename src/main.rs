#![allow(non_snake_case)]

use std::collections::BTreeMap;
use std::collections::HashMap;

use std::env;

use std::error::Error;
use std::result::Result;

mod constants;
mod ctable;
mod parse_csv;

use constants::CITY_CONFIG;
use parse_csv::readTypesCsv;

use crate::ctable::HTMLTable;
use parse_csv::readReqCsv;

#[cfg(all(feature = "nyc", feature = "chi"))]
compile_error!("Only one of `nyc` or `chi` features can be enabled");

#[cfg(not(any(feature = "nyc", feature = "chi")))]
compile_error!("`nyc` or `chi` must be selected");

fn main() -> Result<(), Box<dyn Error>> {
    // get command line arguments
    let args: Vec<String> = env::args().collect();

    // println!("Program arguments: {:?}", args);
    // println!("args count: {}", args.len());
    // // argument validations
    // if args.len() < 2 || args.len() > 4 {
    //     eprintln!("Usage: ./program_name <fromDate> <toDate> ");
    //     return Err("Not enough arguments".into());
    // } else if args[1].parse::<i32>().is_err() || args[2].parse::<i32>().is_err() {
    //     eprintln!("Error: <fromDate> and <toDate> must be valid years");
    //     return Err("Invalid year format".into());
    // } else if args[2] < args[1] {
    //     eprintln!("Error: <toDate> must be greater than or equal to <fromDate>");
    //     return Err("Invalid date range".into());
    // }

    // structures for query 1
    let mut typesByAcronym: HashMap<String, String> = HashMap::new();
    let mut typesByAgencyBySize: BTreeMap<String, BTreeMap<String, i32>> = BTreeMap::new();

    // structures for query 2
    let mut boroughLatLngBySize: BTreeMap<(String, i32, i32), i32> = BTreeMap::new();

    // structures for query 3
    let mut agencyByYearByMonthBySize: BTreeMap<String, BTreeMap<i32, BTreeMap<i32, i32>>> =
        BTreeMap::new();

    // structures for query 4
    let mut promPerQuad: Vec<Vec<u32>> = Vec::new();

    // QUERY 1 - read csv files
    readTypesCsv(
        CITY_CONFIG.typesFilePath,
        &mut typesByAcronym,
        &mut typesByAgencyBySize,
    )?;

    readReqCsv(
        CITY_CONFIG.requestsFilePath,
        &mut typesByAcronym,            // for query 1
        &mut typesByAgencyBySize,       // for query 1
        &mut boroughLatLngBySize,       // for query 2
        &mut agencyByYearByMonthBySize, // for query 3
        &mut promPerQuad,               // for query 4
    )?;

    // ######## PRINTS ########

    //print for query1
    typesByAgencyBySize.iter().for_each(|(infr, b)| {
        b.iter()
            .for_each(|(agency, v)| println!("{} ({}) - {}", infr, agency, v))
    });

    println!("----------------------------------");
    println!("----------------------------------");
    println!("----------------------------------");
    println!("----------------------------------");
    println!("----------------------------------");

    //print for query2
    boroughLatLngBySize
        .iter()
        .for_each(|((borough, lat, lng), v)| println!("{};{};{};{}", borough, lat, lng, v));

    println!("----------------------------------");
    println!("----------------------------------");
    println!("----------------------------------");
    println!("----------------------------------");
    println!("----------------------------------");

    //print for query3
    agencyByYearByMonthBySize
        .iter()
        .for_each(|(agency, year_map)| {
            if let Some((year, month_map)) = year_map.iter().next_back() {
                let mut ytd = 0;
                for month in 1..=12 {
                    if let Some(count) = month_map.get(&month) {
                        ytd += count;
                        println!("{};{};{};{}", agency, year, month, ytd);
                    }
                }
            }
        });

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
