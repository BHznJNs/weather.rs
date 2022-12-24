use std::env::current_exe;
use std::path::Path;
use std::error::Error;

pub fn get_base(name: &str) -> Result<String, Box<dyn Error>> {
    let base = current_exe()?;
    let path = Path::new(base.to_str().unwrap());
    let parent = path.parent();
    
    if let Some(p) = parent {
        let conf_path = p.join(name);
        Ok(
            conf_path
            .to_str()
            .unwrap()
            .to_string()
        )
    } else {
        Ok(String::from(name))
    }
}