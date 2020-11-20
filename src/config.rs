use snafu::ResultExt;

use crate::types::{Config, Result, FileOpenError, TomlParsingError};
use clap::{Arg, App};
use std::fs::File;
use std::io::{BufReader,Read};

pub fn read_config(program: &str) -> Result<Config> {
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");
  let matches = App::new(program).version(VERSION).author("Cohomology")
                                 .arg(Arg::with_name("config").short("c")
                                                              .long("config")
                                                              .value_name("FILE")
                                                              .takes_value(true))
                                 .get_matches();
  let filename = matches.value_of("config").unwrap_or("/etc/simplemm.conf");
  let file = File::open(filename).context(FileOpenError { filename })?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents).context(FileOpenError { filename })?; 
  let config = toml::from_str(&contents).context(TomlParsingError { filename })?;
  Ok(config)
}
