use std::fs::File;
use std::io::{Read, Write};
use zip::write::FileOptions;
use zip::ZipWriter;

pub fn archive() -> std::io::Result<()> {
    let path = "/Users/eboniequ/files/data.txt";
    let zip_path = "/Users/eboniequ/files/archive.zip";

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let zip_file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(zip_file);

    zip.start_file("data.txt", FileOptions::default())?;
    zip.write_all(&buffer)?;

    zip.finish()?;
    println!("Файл {} запакован в {}", path, zip_path);
    
    Ok(())
}
