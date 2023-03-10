use std::ffi::OsString;
use std::fs::{read_dir, File};
use std::io::{BufReader, ErrorKind};
use std::path::{Path, PathBuf};
use std::{env, io};

use serde::de::DeserializeOwned;

use crate::errors::AppError;

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

pub fn parse_json_from_file<P, T>(file_path: P) -> Result<T, AppError>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let file = File::open(file_path).map_err(|e| AppError::ParseJsonError(e.to_string()))?;
    let reader = BufReader::new(file);

    let u: T =
        serde_json::from_reader(reader).map_err(|e| AppError::ParseJsonError(e.to_string()))?;

    Ok(u)
}

#[cfg(test)]
mod test {
    use super::parse_json_from_file;
    use claim::assert_ok;
    use serde::Deserialize;

    #[test]
    fn test_parse_json_from_file() {
        #[derive(Debug, Deserialize)]
        struct Test {
            test: String,
        }

        let result: Test = assert_ok!(parse_json_from_file("src/test/test.json"));
        assert_eq!(result.test, "123");
    }
}
