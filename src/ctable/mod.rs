use libc::{c_char, c_uint, c_void};

unsafe extern "C" {
    pub fn newTable(fileName: *const c_char, columns: c_uint, ...) -> *mut c_void;
    pub fn addHTMLRow(table: *mut c_void, ...);
    pub fn closeHTMLTable(table: *mut c_void);
}
