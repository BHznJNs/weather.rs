use crate::read_config::check_conf::check_conf;

use configparser::ini::Ini;
use std::error::Error;
use std::env::current_dir;

pub fn read_config( key: &mut Option<String>, city: &mut Option<String>) -> Result<(), Box<dyn Error>> {
    let path = current_dir()?;
    let conf_path = format!("{}/config.ini", path.display());

    check_conf(conf_path.as_str())?;

    let mut config = Ini::new();
    let __map = config.load(conf_path)?;
    
    *key = config.get("default", "key");
    *city = config.get("default", "city");
    Ok(())
}