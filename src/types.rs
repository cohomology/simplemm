use serde::Deserialize;
use snafu::Snafu;

#[derive(Deserialize)]
pub struct Config {
    pub db_url : String,
    pub uid : u32,
    pub gid : u32,
    pub pid_file : String,
}

#[derive(Debug,Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Could not open config from {}: {}", filename, source))]
    FileOpenError {
        filename: String,
        source: std::io::Error
    },
    #[snafu(display("Could not parse configuration file {}: {}", filename, source))]
    TomlParsingError {
        filename: String,
        source: toml::de::Error
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
