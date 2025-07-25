use std::fs::File;
use std::io::{BufReader, BufRead, Result};

// -> Result<()> 
// significa que puede devolver una tupla vacia () si todo sale bien, o un error si algo falla

fn main() -> Result<()> {
    // Abrir el archivo
    let file = File::open("/Users/pedrogonzaleznunez/Documents/GitHub/TPE-PI-RUST/resources/Dataset Alumnos/typesCHI.csv")?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line?; // obtengo la linea como un Result<String>
        let fields: Vec<&str> = line.split(',').collect();

        if i == 0 {
            println!("{:?}",fields);
            println!("{}",'\n');
        } else {
            println!("{} {:?}", i, fields);
        }
    }

    Ok(())
}

