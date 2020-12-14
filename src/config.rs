use crate::{error, types};

use std::fs::File;
use std::io::{BufReader, Read};

use snafu::ResultExt;

pub fn read_config(filename: &str) -> error::Result<types::Config> {
    let file = File::open(filename).context(error::FileOpenError { filename })?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .context(error::FileOpenError { filename })?;
    let config = toml::from_str(&contents).context(error::TomlParsingError { filename })?;
    Ok(config)
}
