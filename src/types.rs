use serde::Deserialize;
use snafu::Snafu;

#[derive(Deserialize)]
pub struct Config {
    pub db_url : String,
    pub uid : u32,
    pub gid : u32,
    pub pid_file : String,
    pub working_dir : String,
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
    },
    #[snafu(display("Could not reach database: {}", source))]
    DbConnectionError {
        source: diesel::result::ConnectionError
    },
    #[snafu(display("Could not daemonize: {}", source))]
    DaemonizeError {
        source: daemonize::DaemonizeError
    },
    #[snafu(display("Error connecting to syslog: {}", source))]
    SyslogError {
        source: syslog::Error
    },
    #[snafu(display("Error when initializing the logging subsystem: {}", source))]
    SetLoggerError {
        source: log::SetLoggerError
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
