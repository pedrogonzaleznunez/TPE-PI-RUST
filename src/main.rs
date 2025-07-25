use std::fs::File;
use std::io::{BufRead, BufReader, Result};

// -> Result<()>
// significa que puede devolver una tupla vacia () si todo sale bien, o un error si algo falla

fn main() -> Result<()> {
    // Abrir el archivo
    let root = std::env::current_dir()?;
    let path = root.join("resources/Dataset Alumnos/typesCHI.csv");
    println!("{}", path.display());
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line?; // obtengo la linea como un Result<String>
        let fields: Vec<&str> = line.split(',').collect();

        if i == 0 {
            println!("{:?}", fields);
            println!("{}", '\n');
        } else {
            println!("{} {:?}", i, fields);
        }
    }

    Ok(())
}
