use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct PersistentData {
    pub last_db_path: String,
}

const FILE_NAME: &str = "persistent_data.json";

impl PersistentData {
    pub fn new_empty() -> PersistentData {
        PersistentData {
            last_db_path: "".to_string(),
        }
    }

    // Creates a new empty data if one doesn't already exist
    pub fn new_from_file() -> Result<PersistentData, String> {
        // load from file
        let path: PathBuf = Self::get_dir()?;

        // Append file information
        let path: PathBuf = path.with_file_name(FILE_NAME);
        let path: &str = path.to_str().unwrap();

        let data: String = match std::fs::read_to_string(path) {
            Ok(v) => v,
            Err(v) => {
                return Err(format!("{v}"));
            }
        };

        match serde_json::from_str::<PersistentData>(&data) {
            Ok(v) => {
                return Ok(v);
            }
            Err(v) => {
                return Err(format!("{v}"));
            }
        };
    }

    pub fn set_last_db(&mut self, last_db: &str) {
        self.last_db_path = last_db.to_string();
        let json: String = serde_json::to_string(self).unwrap();

        let write_path: PathBuf = match Self::get_dir() {
            Ok(v) => v,
            Err(v) => {
                eprintln!("{v}");
                return;
            }
        };

        // Create directory
        std::fs::create_dir_all(&write_path).unwrap();

        // Append file information
        let write_path: PathBuf = write_path.with_file_name(FILE_NAME);
        let write_path: &str = write_path.to_str().unwrap();

        // Write file
        match fs::write(write_path, json) {
            Ok(()) => println!("Wrote persistent data file to {write_path}"),
            Err(v) => eprintln!("Error writing persistent data to path {write_path} error: {v}"),
        };
    }

    fn get_dir() -> Result<PathBuf, String> {
        let proj_dirs = match ProjectDirs::from("com", "Ryt", "LastFinance") {
            Some(v) => v,
            None => return Err("No place to save project data".to_string()),
        };

        return Ok(proj_dirs.data_local_dir().to_path_buf());
    }
}
