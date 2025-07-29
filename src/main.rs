#![allow(non_snake_case)]

use std::collections::BTreeMap;
use std::collections::HashMap;

// use std::env;

use std::error::Error;
use std::path::PathBuf;
use std::result::Result;

mod constants;
mod csv;
mod ctable;

use constants::CITY_CONFIG;
use ctable::HTMLTable;

use csv::CSVFile;
use csv::readReqCsv;
use csv::readTypesCsv;

#[cfg(all(feature = "nyc", feature = "chi"))]
compile_error!("Only one of `nyc` or `chi` features can be enabled");

#[cfg(not(any(feature = "nyc", feature = "chi")))]
compile_error!("`nyc` or `chi` must be selected");

fn main() -> Result<(), Box<dyn Error>> {
    // get command line arguments
    // let args: Vec<String> = env::args().collect();

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
    )?;

    // ######## Writes to output files ########

    // ======= Query 1 =======
    let csv_file_q1 = CSVFile {
        path: PathBuf::from("query1.csv"),
    };
    let rows_q1 = typesByAgencyBySize.iter().flat_map(|(infr, agencies)| {
        agencies
            .iter()
            .map(|(agency, count)| ([infr, agency, count] as [&dyn std::fmt::Display; 3]))
    });

    csv_file_q1.write_file(&vec!["type", "agency", "requests"], rows_q1)?;
    // ======= END Query 1 =======

    // ======= Query 2 =======
    let csv_file_q2 = CSVFile {
        path: PathBuf::from("query2.csv"),
    };
    let rows_q2 = boroughLatLngBySize
        .iter()
        .map(|((borough, lat, long), count)| {
            [borough, lat, long, count] as [&dyn std::fmt::Display; 4]
        });
    csv_file_q2.write_file(
        &vec!["neighborhood", "quatLat", "quadLon", "request"],
        rows_q2,
    )?;
    // ======= END Query 2 =======

    // ======= Query 3 =======
    let csv_file_q3 = CSVFile {
        path: PathBuf::from("query3.csv"),
    };
    let rows_q3 = agencyByYearByMonthBySize
        .iter()
        .flat_map(move |(agency, year_map)| {
            year_map.iter().flat_map(move |(year, month_resolved_map)| {
                month_resolved_map
                    .iter()
                    .scan(0, move |state, (month, ytd)| {
                        *state += ytd;
                        Some(((agency, year, month, *state), *state))
                    })
                    .map(move |((agency, year, month, ytd), _)| {
                        [
                            Box::new(agency),
                            Box::new(year),
                            Box::new(month),
                            Box::new(ytd),
                        ] as [Box<dyn std::fmt::Display>; 4]
                    })
            })
        });

    csv_file_q3.write_file(&vec!["agency", "year", "month", "resolvedYTD"], rows_q3)?;
    // ======= END Query 3 =======

    // HTML output 👇
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
