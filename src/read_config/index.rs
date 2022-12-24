use crate::read_config::check_conf::check_conf;
use crate::read_config::get_base::get_base;

use configparser::ini::Ini;
use std::error::Error;

pub fn read_config( key: &mut Option<String>, city: &mut Option<String>) -> Result<(), Box<dyn Error>> {
    let conf_path = get_base("./config.ini")?;

    check_conf(conf_path.as_str())?;

    let mut config = Ini::new();
    let __map = config.load(conf_path)?;
    
    *key = config.get("default", "key");
    *city = config.get("default", "city");
    Ok(())
}