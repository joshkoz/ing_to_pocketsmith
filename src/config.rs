use std::path::PathBuf;

use crate::prelude::*;

pub struct Config {
    pub path: PathBuf,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config> {
        args.next();

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err(Error::Generic(String::from("Didn't get a path"))),
        };
        let path = PathBuf::from(path.as_str());

        if !path.exists() {
            let msg = format!("Path {path} doesn't exist", path = path.display());
            return Err(Error::Generic(msg));
        }

        Ok(Config { path })
    }
}
