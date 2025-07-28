use libc::{c_char, c_uint, c_void};

unsafe extern "C" {
    pub fn newTable(fileName: *const c_char, columns: c_uint, ...) -> *mut c_void;
    pub fn addHTMLRow(table: *mut c_void, ...);
    pub fn closeHTMLTable(table: *mut c_void);
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
    fn testCTable() -> Result<(), Box<dyn Error>> {
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
        }

        Ok(())
    }
}
