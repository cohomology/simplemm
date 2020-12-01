use crate::types::Config;
use crate::error::{Result, FileOpenError, TomlParsingError};

use std::fs::File;
use std::io::{BufReader,Read};

use snafu::ResultExt;


pub fn read_config(filename: &str) -> Result<Config> {
  let file = File::open(filename).context(FileOpenError { filename })?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents).context(FileOpenError { filename })?; 
  let config = toml::from_str(&contents).context(TomlParsingError { filename })?;
  Ok(config)
}
