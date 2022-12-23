use crate::read_config::check_conf::check_conf;

use configparser::ini::Ini;

pub fn read_config( key: &mut Option<String>, city: &mut Option<String>) {
    const CONF_PATH: &str = "./config.ini";

    let is_conf_created = check_conf(CONF_PATH);
    match is_conf_created {
        Ok(_s) => {},
        Err(_e) => {
            panic!("Config file create error!");
        }
    }

    let mut config = Ini::new();
    let __map = config.load(CONF_PATH);

    match __map {
        Ok(_map) => {
            *key = config.get("default", "key");
            *city = config.get("default", "city");
        },
        Err(_map) => {
            panic!("Open config file error.");
        }
    }
}