use std::{error::Error, path::PathBuf};

use dirs;

pub fn get_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut s| {
        s.push("rusty-belt");
        s
    })
}

pub fn get_data_path() -> Option<PathBuf> {
    dirs::data_dir().map(|mut s| {
        s.push("rusty-belt");
        s
    })
}

pub fn handle_file_presence(pb: &PathBuf) -> Result<(), Box<dyn Error>> {
    pb.try_exists()
        .map_err(|e| Box::<dyn Error>::from(e))
        .and_then(|flag| {
            if flag {
                Ok(())
            } else {
                Err(Box::<dyn Error>::from(format!(
                    "Can't find file. Expect it here: \n {}",
                    pb.as_path().display()
                )))
            }
        })
}
