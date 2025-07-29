#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Result, Write};

use std::collections::BTreeMap;
use std::collections::HashMap;

use std::path::PathBuf;

use super::constants::CITY_CONFIG;

enum TypesFields {
    Name = 0,
    Acronym = 1,
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    InProgress,
    Closed,
    Open,
}

impl std::str::FromStr for Status {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Open" => Ok(Status::Open),
            "In Progress" => Ok(Status::InProgress),
            "Closed" => Ok(Status::Closed),
            "Completed" => Ok(Status::Closed), // assuming "Completed" is treated as "Closed"
            "Canceled" => Ok(Status::InProgress), // assuming "Canceled" is treated as "InProgress"
            "Started" => Ok(Status::InProgress), // assuming "Started" is treated as "InProgress"
            "Pending" => Ok(Status::InProgress), // assuming "Pending" is treated as "InProgress"
            "Assigned" => Ok(Status::InProgress), // assuming "Assigned" is treated as "InProgress"
            _ => Err(()),
        }
    }
}

pub struct CSVFile {
    pub path: PathBuf,
}

impl CSVFile {
    pub fn parse_file<F: FnMut(Vec<&str>) -> ()>(&self, mut f: F) -> Result<()> {
        let path = self.getPath()?;

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            if i == 0 {
                continue; // header line, not validated
            }

            let line = line?;

            let fields: Vec<&str> = line.split(';').collect();
            f(fields);
        }

        Ok(())
    }

    pub fn write_file<'a, Iter, Row, D>(&self, headers: &[&str], data: Iter) -> std::io::Result<()>
    where
        Iter: IntoIterator<Item = Row>,
        Row: IntoIterator<Item = D>,
        D: std::fmt::Display,
    {
        let path = self.getPath()?;
        let file = File::create(path)?;
        let mut writer = LineWriter::new(file);
        writeln!(writer, "{}", headers.join(";"))?;

        let mut data_iter = data.into_iter().peekable();

        while let Some(row) = data_iter.next() {
            let mut iter = row.into_iter().peekable();
            while let Some(cell) = iter.next() {
                write!(writer, "{}", cell)?;
                if iter.peek().is_some() {
                    write!(writer, ";")?;
                }
            }

            if data_iter.peek().is_some() {
                writeln!(writer)?;
            }
        }

        writer.flush()?;
        Ok(())
    }

    fn getPath(&self) -> Result<PathBuf> {
        let root = std::env::current_dir()?;
        Ok(root.join(&self.path))
    }
}

pub fn readTypesCsv(
    filePath: &str,
    typesByAcronym: &mut HashMap<String, String>,
    typesByAgencyBySize: &mut BTreeMap<String, BTreeMap<String, i32>>,
) -> Result<()> {
    let csv_file = CSVFile {
        path: PathBuf::from(filePath),
    };

    csv_file.parse_file(|fields| {
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
    })?;

    Ok(())
}

pub fn readReqCsv(
    filePath: &str,
    typesByAcronym: &mut HashMap<String, String>,
    typesByAgencyBySize: &mut BTreeMap<String, BTreeMap<String, i32>>,
    boroughLatLngBySize: &mut BTreeMap<(String, i32, i32), i32>,
    agencyByYearByMonthBySize: &mut BTreeMap<String, BTreeMap<i32, BTreeMap<i32, i32>>>,
    fromToDates: &mut Vec<i32>,
    promPerQuad: &mut BTreeMap<(u32, u32), i32>,
) -> Result<()> {
    let csv_file = CSVFile {
        path: PathBuf::from(filePath),
    };

    csv_file.parse_file(|fields| {
        let agencyName: &str = fields[CITY_CONFIG.requestCSVFields.AgencyName as usize];
        let acronym: &str = fields[CITY_CONFIG.requestCSVFields.Acronym as usize];
        let borough: &str = fields[CITY_CONFIG.requestCSVFields.Borough as usize];
        let lat: f64 = fields[CITY_CONFIG.requestCSVFields.Latitude as usize]
            .parse()
            .expect("Latitude must be a valid number");
        let lng: f64 = fields[CITY_CONFIG.requestCSVFields.Longitude as usize]
            .parse()
            .expect("Longitude must be a valid number");
        // get the quadrant from lat and lng
        let (lat_quadrant, lng_quadrant) = getquadrantFromLatLng(lat, lng);

        let date_parts: Vec<&str> = fields[CITY_CONFIG.requestCSVFields.CreatedDate as usize]
            .split('-')
            .collect();

        let year: i32 = date_parts
            .get(0)
            .and_then(|s| s.parse::<i32>().ok())
            .expect("Year must be a valid number");

        let month: i32 = date_parts
            .get(1)
            .and_then(|s| s.parse::<i32>().ok())
            .expect("Month must be a valid number");

        let typeName: &str = typesByAcronym
            .get(acronym)
            .expect("Type name must be defined");

        let status: Status = fields[CITY_CONFIG.requestCSVFields.Status as usize]
            .parse()
            .expect("Status cannot be parsed");

        // data for query1
        typesByAgencyBySize
            .entry(typeName.to_string())
            .or_insert_with(BTreeMap::new)
            .entry(agencyName.to_string())
            .and_modify(|count: &mut i32| *count += 1)
            .or_insert(1);

        // data for query2
        boroughLatLngBySize
            .entry((borough.to_string(), lat_quadrant, lng_quadrant))
            .and_modify(|count: &mut i32| *count += 1)
            .or_insert(1);

        if status == Status::Closed {
            // data for query3
            agencyByYearByMonthBySize
                .entry(agencyName.to_string())
                .or_insert_with(BTreeMap::new)
                .entry(year)
                .or_insert_with(BTreeMap::new)
                .entry(month)
                .and_modify(|count: &mut i32| *count += 1)
                .or_insert(1);

            // data for query4

            match fromToDates.len() as i32 {
                1 => {
                    // = 1 one arg --> toDate
                    if year <= fromToDates[0] {
                        promPerQuad
                            .entry((lat_quadrant as u32, lng_quadrant as u32))
                            .and_modify(|count: &mut i32| *count += 1)
                            .or_insert(1);
                    }
                }
                2 => {
                    // = 2 two args --> fromDate and toDate
                    if year >= fromToDates[0] && year <= fromToDates[1] {
                        promPerQuad
                            .entry((lat_quadrant as u32, lng_quadrant as u32))
                            .and_modify(|count: &mut i32| *count += 1)
                            .or_insert(1);
                    }
                }
                _ => {
                    // nothing to do crazy piter
                }
            }
        }
    })?;

    Ok(())
}

// aux method for query 2
fn getquadrantFromLatLng(lat: f64, lng: f64) -> (i32, i32) {
    const QUADRANT_SIZE: f64 = 0.1;
    let lat_quadrant = (lat / QUADRANT_SIZE).floor() as i32;
    let lng_quadrant = (lng / QUADRANT_SIZE).floor() as i32;

    (lat_quadrant, lng_quadrant)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use temp_dir::TempDir;

    #[test]
    fn write_csv() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let csv_file = CSVFile {
            path: temp_dir.path().join("test_write.csv"),
        };
        let iter = vec![
            vec!["Row 1 Col 1", "Row 1 Col 2"],
            vec!["Row 2 Col 1", "Row 2 Col 2"],
        ]
        .into_iter();
        csv_file.write_file(&vec!["Header 1", "Header 2"], iter)?;
        assert_eq!(
            read_to_string(csv_file.getPath().unwrap()).unwrap(),
            "Header 1;Header 2\nRow 1 Col 1;Row 1 Col 2\nRow 2 Col 1;Row 2 Col 2"
        );
        Ok(())
    }

    #[test]
    fn read_csv() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let csv_file = CSVFile {
            path: temp_dir.path().join("test_read.csv"),
        };
        let data = vec![
            "Header 1;Header 2",
            "Row 1 Col 1;Row 1 Col 2",
            "Row 2 Col 1;Row 2 Col 2",
        ]
        .join("\n");

        std::fs::write(csv_file.getPath().unwrap(), data)?;

        let mut rows = Vec::new();
        csv_file.parse_file(|fields| {
            rows.push(
                fields
                    .iter()
                    .map(|&s| s.to_string())
                    .collect::<Vec<String>>(),
            );
        })?;

        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows[0],
            vec!["Row 1 Col 1".to_string(), "Row 1 Col 2".to_string()]
        );
        assert_eq!(
            rows[1],
            vec!["Row 2 Col 1".to_string(), "Row 2 Col 2".to_string()]
        );

        Ok(())
    }
}
