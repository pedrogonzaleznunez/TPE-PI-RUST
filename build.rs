fn main() {
    cc::Build::new()
        .file("src/ctable/htmlTable.c")
        .flag("-Wno-sign-compare")
        .include("src/ctable")
        .compile("clibrary");

    println!("cargo:rerun-if-changed=src/ctable/htmlTable.c");
    println!("cargo:rerun-if-changed=src/ctable/htmlTable.h");
}
