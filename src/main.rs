#![allow(non_snake_case)]

use std::collections::BTreeMap;
use std::collections::HashMap;

// use std::env;

use std::env;
use std::error::Error;
use std::i32;
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
    let args: Vec<String> = env::args().collect();
    let args_count = args.len();
    let mut fromToDates: Vec<i32> = Vec::new();

    // println!("Program arguments: {:?}", args);
    // println!("args count: {}", args_count);

    // argument validations
    match args_count {
        0..=1 => {
            eprintln!("Error: Not enough arguments");
            return Err("Not enough arguments".into());
        }
        2 => {
            if validateYear(&args[1]).is_err() {
                eprintln!("Error: <toDate> must be a valid year");
            } else {
                fromToDates.push(args[1].parse::<i32>().unwrap());
            }
        }
        3 => {
            if validateYear(&args[1]).is_err() || validateYear(&args[2]).is_err() {
                eprintln!("Error: <fromDate> and <toDate> must be valid years");
            } else if &args[1] > &args[2] {
                eprintln!("Error: <fromDate> must be less than or equal to <toDate>");
            } else {
                fromToDates.push(args[1].parse::<i32>().unwrap());
                fromToDates.push(args[2].parse::<i32>().unwrap());
            }
        }
        4..=usize::MAX => {
            eprintln!("Error: Too many arguments");
            std::process::exit(1);
        }
        _ => {
            eprintln!("Error: Invalid number of arguments");
        }
    }

    // structures for query 1
    let mut typesByAcronym: HashMap<String, String> = HashMap::new();
    let mut typesByAgencyBySize: BTreeMap<String, BTreeMap<String, i32>> = BTreeMap::new();

    // structures for query 2
    let mut boroughLatLngBySize: BTreeMap<(String, i32, i32), i32> = BTreeMap::new();

    // structures for query 3
    let mut agencyByYearByMonthBySize: BTreeMap<String, BTreeMap<i32, BTreeMap<i32, i32>>> =
        BTreeMap::new();

    // structures for query 4
    let mut promPerQuad: BTreeMap<(u32, u32), i32> = BTreeMap::new();

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
        &mut fromToDates,               // prog args for queries 4,5
        &mut promPerQuad,               // for query 4
    )?;

    // ######## Writes to output files ########

    // ======= Query 1 =======
    let csv_file_q1 = CSVFile {
        path: PathBuf::from("query1.csv"),
    };
    let rows_q1 = typesByAgencyBySize.iter().flat_map(|(infr, agencies)| {
        agencies
            .iter()
            .map(|(agency, count)| [infr, agency, count] as [&dyn std::fmt::Display; 3])
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

    // ======= Query 4 =======
    let csv_file_q4 = CSVFile {
        path: PathBuf::from("query4.csv"),
    };

    let mut rows_q4: Vec<[String; 3]> = Vec::new();

    for ((lat, lon), &_) in &promPerQuad {
        let mut total = 0;
        let lat_i32 = *lat as i32;
        let lon_i32 = *lon as i32;

        for dlat in -1..=1 {
            for dlon in -1..=1 {
                let neighbor = ((lat_i32 + dlat) as u32, (lon_i32 + dlon) as u32);
                total += promPerQuad.get(&neighbor).unwrap_or(&0);
            }
        }

        let avg = (total as f64 / 9.0).trunc();
        if avg > 0.0 {
            rows_q4.push([lat.to_string(), lon.to_string(), format!("{:.2}", avg)]);
        }
    }

    csv_file_q4.write_file(&vec!["quadLat", "quadLon", "resolvedAvg"], rows_q4)?;
    // ======= END Query 4 =======

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

pub fn validateYear(year: &str) -> Result<(), String> {
    if year.len() != 4 {
        return Err("Year must be a 4-digit number".to_string());
    }

    let year_num: i32 = year.parse().map_err(|_| "Year must be a valid number")?;

    if year_num < 1900 || year_num > 2100 {
        return Err("Year must be between 1900 and 2100".to_string());
    }

    Ok(())
}
