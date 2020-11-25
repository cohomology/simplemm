use serde::{Serialize, Deserialize};
use snafu::Snafu;

#[derive(Serialize,Deserialize)]
pub struct Config {
    pub db_url : String,
    pub uid : u32,
    pub gid : u32,
    pub pid_file : String,
    pub working_dir : String,
    pub socket : String,
}

#[derive(Serialize,Deserialize)]
pub enum Action {
    Stop,
    Subscribe,
    Unsubscribe,
    Send
}

#[derive(Serialize,Deserialize)]
pub struct Command {
    pub action : Action,
    pub originator : String,
    pub data : String,
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
    #[snafu(display("Could not connect to syslog: {}", source))]
    SyslogError {
        source: syslog::Error
    },
    #[snafu(display("Error when initializing the logging subsystem: {}", source))]
    SetLoggerError {
        source: log::SetLoggerError
    },
    #[snafu(display("Path {} not writeable", path))]
    CouldNotWriteToFileOrDirectory {
        path : String
    },
    #[snafu(display("Path {} is not a file", path))]
    PathNoFile {
        path : String
    },
    #[snafu(display("Error obtaining path metadata for {}: {}", path, source))]
    PathMetadataError {
        path : String,
        source : std::io::Error
    },
    #[snafu(display("Could not bind to socket {}: {}", path, source))]
    SocketBindError {
        path : String,
        source : std::io::Error
    },
    #[snafu(display("Could not bind exit handler: {}", source))]
    ExitHandlerError {
        source : ctrlc::Error
    },
    #[snafu(display("Could not parse request: {}", source))]
    RequestParseError {
        source : serde_json::Error
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
