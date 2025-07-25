#![allow(non_snake_case)]

use std::collections::HashMap;
use std::collections::BTreeMap;

use std::error::Error;
use std::result::Result;

mod parse_csv;
use parse_csv::readTypesCsv;

const TYPES_NYC_PATH: &str = "resources/Dataset Alumnos/typesNYC.csv";
const TYPES_CHI_PATH: &str = "resources/Dataset Alumnos/typesCHI.csv";
// -> Result<()>


// means it may return an empty tuple () if everything's alright, or an error if something went wrong

fn main() -> Result<(), Box<dyn Error>> {

    // maps for chi
    let mut typesByAcronymCHI: HashMap<String, String> = HashMap::new();
    let typesByAgencyBySizeCHI: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    // maps for nyc
    let mut typesByAcronymNYC: HashMap<String, String> = HashMap::new();
    let typesByAgencyBySizeNYC: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    
    //first we read types
    readTypesCsv(TYPES_NYC_PATH,&mut typesByAcronymCHI)?;
    readTypesCsv(TYPES_CHI_PATH,&mut typesByAcronymNYC)?;


    typesByAcronymNYC.iter().for_each(|f| println!("{:?}", f));

    Ok(())
}


