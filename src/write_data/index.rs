use std::fs::File;
use std::io;
use std::io::Write;

pub fn write_data(
    path: String, data: String
) -> Result<(), io::Error> {
    
    let mut buffer = File::create(path)?;
    write!(buffer, "{}", data)?;
    Ok(())
}