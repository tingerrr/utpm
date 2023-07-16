use std::{
    env::current_dir,
    fs::{read, read_dir, symlink_metadata},
};

use dirs::data_dir;

use super::state::ErrorState;


pub fn get_data_dir() -> String {
    match data_dir() {
        Some(dir) => match dir.to_str() {
            Some(string) => String::from(string),
            None => String::from("/.local/share"), //default on linux
        },
        None => String::from("/.local/share"),
    }
}


pub fn d_local() -> String {
    get_data_dir() + "/typst/packages/local"
}


pub fn get_current_dir() -> Result<String, ErrorState> {
    match current_dir() {
        Ok(val) => match val.to_str() {
            Some(v) => Ok(String::from(v)),
            None => Err(ErrorState::CurrentDirectoryError(String::from(
                "there is no current directory",
            ))),
        },
        Err(val) => Err(ErrorState::CurrentDirectoryError(val.to_string())),
    }
}

pub fn current_package() -> Result<String, ErrorState> {
    Ok(get_current_dir()? + "/typst.toml")
}


pub fn check_path_dir(path: &String) -> bool {
    read_dir(path).is_ok()
}

pub fn check_path_file(path: &String) -> bool {
    read(path).is_ok()
}

pub fn check_existing_symlink(path: &String) -> bool {
    let x = match symlink_metadata(path) {
        Ok(val) => val,
        Err(_) => return false,
    };
    x.file_type().is_symlink()
}
