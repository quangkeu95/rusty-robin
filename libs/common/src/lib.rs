use std::ffi::OsString;
use std::fs::read_dir;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::{env, io};

/// Get the project root (relative to closest Cargo.lock file)
pub fn get_project_root() -> io::Result<PathBuf> {
    let path = env::current_dir()?;
    let path_ancestors = path.as_path().ancestors();

    let cargo_file = OsString::from("Cargo.lock");
    for p in path_ancestors {
        let has_cargo = read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == cargo_file);
        if has_cargo {
            return Ok(PathBuf::from(p));
        }
    }
    Err(io::Error::new(
        ErrorKind::NotFound,
        "ran out of places to find Cargo.toml",
    ))
}
