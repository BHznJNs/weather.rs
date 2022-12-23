use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn check_conf(path: &str) -> io::Result<()> {
    let path_obj = Path::new(path);
    if !path_obj.exists() {
        println!("Config file does not exist, creating...");

        let mut buffer = File::create(path)?;
        write!(buffer, "[Default]\n; key = ; 高德天气 API\ncity = 110000")?;
    }
    Ok(())
}