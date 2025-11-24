use std::{fs::{self, File}, io::{self}, path::Path};
use serde::{Serialize, de::DeserializeOwned};

pub enum FileType {
    Json,
    Toml,
}

pub fn load<T>(path: &str, file_type: FileType) -> Result<T, anyhow::Error> where T: Sized + DeserializeOwned {
    match file_type {
        FileType::Json => Ok(read_from_file_json::<T>(path)?),
        FileType::Toml => Ok(read_from_file_toml::<T>(path)?),
    }
}

pub fn save<T>(data: &T, path: &str, file_type: FileType) where T: Sized + Serialize {
    match file_type {
        FileType::Json => write_to_file_json(data, path),
        FileType::Toml => write_to_file_toml(data, path),
    }
}

fn read_from_file_toml<T>(path: &str) -> anyhow::Result<T> where T: DeserializeOwned {
    let file = fs::read_to_string(path)?;
    toml::from_str(file.as_str()).map_err(anyhow::Error::from)
}

fn write_to_file_toml<T>(data: &T, path: &str) where T: Serialize {
    let parent_dir = Path::new(path).parent().unwrap();
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).unwrap();
    }
    let data_string = toml::to_string_pretty(data).unwrap();
    fs::write(path, data_string).unwrap();
}

fn read_from_file_json<T>(path: &str) -> io::Result<T> where T: DeserializeOwned {
    let f = File::open(path)?;
    serde_json::from_reader::<File, T>(f).map_err(io::Error::from)
}

fn write_to_file_json<T>(data: &T, path: &str) where T: Serialize {
    let parent_dir = Path::new(path).parent().unwrap();
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).unwrap();
    }
    let file = File::create(path).unwrap();
    serde_json::to_writer(file, data).unwrap();
}