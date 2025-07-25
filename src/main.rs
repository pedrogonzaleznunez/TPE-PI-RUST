#![allow(non_snake_case)]

use std::collections::BTreeMap;
use std::collections::HashMap;

use std::error::Error;
use std::result::Result;

mod parse_csv;
use parse_csv::readReqCsv;
use parse_csv::readTypesCsv;

const TYPES_NYC_PATH: &str = "resources/Dataset Alumnos/typesNYC.csv";
const TYPES_CHI_PATH: &str = "resources/Dataset Alumnos/typesCHI.csv";

// just a million baby
const REQ_NYC_PATH: &str = "resources/Dataset Alumnos/million/requestsNYC.csv";
const REQ_CHI_PATH: &str = "resources/Dataset Alumnos/million/requestsCHI.csv";

#[cfg(all(feature = "nyc", feature = "chi"))]
compile_error!("Only one of `nyc` or `chi` features can be enabled");

#[cfg(not(any(feature = "nyc", feature = "chi")))]
compile_error!("`nyc` or `chi` must be selected");

// means it may return an empty tuple () if everything's alright, or an error if something went wrong

fn main() -> Result<(), Box<dyn Error>> {
    // maps for chi
    let mut typesByAcronymCHI: HashMap<String, String> = HashMap::new();
    let mut typesByAgencyBySizeCHI: BTreeMap<String, BTreeMap<String, i32>> = BTreeMap::new();

    // maps for nyc
    let mut typesByAcronymNYC: HashMap<String, String> = HashMap::new();
    let mut typesByAgencyBySizeNYC: BTreeMap<String, BTreeMap<String, i32>> = BTreeMap::new();

    //first we read types
    readTypesCsv(
        TYPES_NYC_PATH,
        &mut typesByAcronymNYC,
        &mut typesByAgencyBySizeNYC,
    )?;
    // typesByAcronymNYC.iter().for_each(|f| println!("{:?}", f));
    // readTypesCsv(TYPES_CHI_PATH, &mut typesByAcronymNYC, &mut typesByAgencyBySizeCHI)?;

    readReqCsv(
        REQ_NYC_PATH,
        &mut typesByAcronymNYC,
        &mut typesByAgencyBySizeNYC,
    )?;
    // readReqCsv(REQ_CHI_PATH, &mut typesByAcronymCHI, &mut typesByAgencyBySizeCHI)?;

    typesByAgencyBySizeNYC
        .iter()
        .for_each(|(a, b)| b.iter().for_each(|(k, v)| println!("{}", v)));

    Ok(())
}
