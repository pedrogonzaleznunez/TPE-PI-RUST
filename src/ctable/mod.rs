use std::ffi::CString;
use std::io::Error;

use libc::{c_char, c_uint, c_void};
use std::result::Result;

unsafe extern "C" {
    fn newTable(fileName: *const c_char, columns: c_uint, ...) -> *mut c_void;
    fn addHTMLRow(table: *mut c_void, ...);
    fn closeHTMLTable(table: *mut c_void);
}

pub struct HTMLTable {
    ptr: *mut c_void,
    cols: u8,
}

impl HTMLTable {
    pub fn new(fileName: &str, columns: Vec<&str>) -> Result<HTMLTable, Error> {
        let cFileName = CString::new(fileName).expect("Cannot create CString from table name");

        let c_strings: Vec<CString> = columns
            .iter()
            .map(|c| CString::new(c.to_string()).expect("Cannot create CString from Column name"))
            .collect();
        let cols: Vec<*const c_char> = c_strings.iter().map(|s| s.as_ptr()).collect();

        let ptr = unsafe {
            match columns.len() {
                0 => Err("Cannot create table with 0 headers"),
                1 => Ok(newTable(cFileName.as_ptr(), 1, cols[0])),
                2 => Ok(newTable(cFileName.as_ptr(), 2, cols[0], cols[1])),
                3 => Ok(newTable(cFileName.as_ptr(), 3, cols[0], cols[1], cols[2])),
                4 => Ok(newTable(
                    cFileName.as_ptr(),
                    4,
                    cols[0],
                    cols[1],
                    cols[2],
                    cols[3],
                )),
                5 => Ok(newTable(
                    cFileName.as_ptr(),
                    5,
                    cols[0],
                    cols[1],
                    cols[2],
                    cols[3],
                    cols[4],
                )),
                6 => Ok(newTable(
                    cFileName.as_ptr(),
                    6,
                    cols[0],
                    cols[1],
                    cols[2],
                    cols[3],
                    cols[4],
                    cols[5],
                )),
                7 => Ok(newTable(
                    cFileName.as_ptr(),
                    7,
                    cols[0],
                    cols[1],
                    cols[2],
                    cols[3],
                    cols[4],
                    cols[5],
                    cols[6],
                )),
                8 => Ok(newTable(
                    cFileName.as_ptr(),
                    8,
                    cols[0],
                    cols[1],
                    cols[2],
                    cols[3],
                    cols[4],
                    cols[5],
                    cols[6],
                    cols[7],
                )),
                9.. => Err("Cannot create table with more than 8 headers"),
            }
        };

        match ptr {
            Ok(p) if !p.is_null() => Ok(HTMLTable {
                ptr: p,
                cols: cols.len() as u8,
            }),
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to create HTML table",
            )),
        }
    }

    pub fn add_row(&mut self, row: Vec<&str>) -> Result<(), Error> {
        if row.len() != self.cols as usize {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "Row length {} does not match table columns {}",
                    row.len(),
                    self.cols
                ),
            ));
        }

        let c_strings: Vec<CString> = row
            .iter()
            .map(|c| CString::new(c.to_string()).expect("Cannot create CString from Row element"))
            .collect();
        let c_row: Vec<*const c_char> = c_strings.iter().map(|s| s.as_ptr()).collect();

        unsafe {
            match c_row.len() {
                0 => Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Cannot add empty row",
                )),
                1 => Ok(addHTMLRow(self.ptr, c_row[0])),
                2 => Ok(addHTMLRow(self.ptr, c_row[0], c_row[1])),
                3 => Ok(addHTMLRow(self.ptr, c_row[0], c_row[1], c_row[2])),
                4 => Ok(addHTMLRow(self.ptr, c_row[0], c_row[1], c_row[2], c_row[3])),
                5 => Ok(addHTMLRow(
                    self.ptr, c_row[0], c_row[1], c_row[2], c_row[3], c_row[4],
                )),
                6 => Ok(addHTMLRow(
                    self.ptr, c_row[0], c_row[1], c_row[2], c_row[3], c_row[4], c_row[5],
                )),
                7 => Ok(addHTMLRow(
                    self.ptr, c_row[0], c_row[1], c_row[2], c_row[3], c_row[4], c_row[5], c_row[6],
                )),
                8 => Ok(addHTMLRow(
                    self.ptr, c_row[0], c_row[1], c_row[2], c_row[3], c_row[4], c_row[5], c_row[6],
                    c_row[7],
                )),
                9.. => Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Cannot add row with more than 8 elements",
                )),
            }
        }
    }

    pub fn close(self) -> Result<(), Error> {
        unsafe {
            closeHTMLTable(self.ptr);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::ffi::CString;
    use std::fs::read_to_string;
    use std::result::Result;
    use temp_dir::TempDir;

    const FIXTURE: &'static str = "<link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css\" rel=\"stylesheet\" integrity=\"sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3\" crossorigin=\"anonymous\"><html><table class=\"table table-striped table-hover\"><thead><tr><th>Header 1</th><th>Header 2</th><th>Header 3</th></tr></thead><tbody><tr><td>Elem 1</td><td>Elem 2</td><td>Elem 3</td></tr></tbody></table></html>";

    #[test]
    fn test_unsafe_c_bindings() -> Result<(), Box<dyn Error>> {
        let tmpdir = TempDir::new()?;
        let path = tmpdir.path();
        let fileName = CString::new(path.join("table").display().to_string())
            .expect("Failed to create CString on table tests");

        unsafe {
            let table = newTable(
                fileName.as_ptr(),
                3,
                CString::new("Header 1")?.as_ptr(),
                CString::new("Header 2")?.as_ptr(),
                CString::new("Header 3")?.as_ptr(),
            );
            addHTMLRow(
                table,
                CString::new("Elem 1")?.as_ptr(),
                CString::new("Elem 2")?.as_ptr(),
                CString::new("Elem 3")?.as_ptr(),
            );
            closeHTMLTable(table);

            assert_eq!(FIXTURE, read_to_string(path.join("table"))?);
        };

        Ok(())
    }

    #[test]
    fn test_new_table() -> Result<(), Box<dyn Error>> {
        let tmpdir = TempDir::new()?;
        let path = tmpdir.path();
        let fileName = path.join("test_table");

        let mut table = HTMLTable::new(
            fileName.to_str().unwrap(),
            vec!["Header 1", "Header 2", "Header 3"],
        )?;
        table.add_row(vec!["Elem 1", "Elem 2", "Elem 3"])?;
        table.close()?;

        assert_eq!(FIXTURE, read_to_string(fileName)?);
        Ok(())
    }
}
