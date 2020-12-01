use serde::{Serialize, Deserialize};
use snafu::Snafu;

#[derive(Debug,Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Could not open config file \"{}\": {}", filename, source))]
    FileOpenError {
        filename: String,
        source: std::io::Error
    },
    #[snafu(display("Could not parse configuration file \"{}\": {}", filename, source))]
    TomlParsingError {
        filename: String,
        source: toml::de::Error
    },
    #[snafu(display("Could not reach database: {}", source))]
    DbConnectionError {
        source: diesel::result::ConnectionError
    },
    #[snafu(display("Could not daemonize: {}. Server already running?", source))]
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
    #[snafu(display("Path \"{}\" not writeable. Permission error?", path))]
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
    },
    #[snafu(display("Could not serialize request: {}", source))]
    RequestSerializeError {
        source : serde_json::Error
    }, 
    #[snafu(display("Could not read/write server state: {}", source))]
    ServerStateError {
        source : Box<dyn std::error::Error>
    }, 
    #[snafu(display("Could not read pid file \"{}\": {}. Server not running?", filename, source))]
    PidFileReadError {
        filename : String, 
        source : std::io::Error
    },  
    #[snafu(display("Could not parse pid file \"{}\": {}. Server not running?", filename, source))]
    PidFileParseError {
        filename : String,
        source : std::num::ParseIntError
    },   
    #[snafu(display("Could not connect to socket {}: {}. Server not running?", socket, source))]
    SocketConnectError {
        socket : String,
        source : std::io::Error
    },    
    #[snafu(display("Could not change permissions of socket {}: {}", socket, source))]
    SocketPermissionError {
        socket : String,
        source : std::io::Error
    },     
    #[snafu(display("Could not close socket {}: {}", socket, source))]
    SocketCloseError {
        socket : String,
        source : std::io::Error
    },       
}
 
#[derive(Clone,Serialize,Deserialize)]
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
    Alive
}

#[derive(Serialize,Deserialize)]
pub struct Command {
    pub action : Action,
    pub originator : String,
    pub data : Option<String>,
}

#[derive(Clone,Serialize,Deserialize)] 
pub struct DaemonState {
    pub config : Config,
    pub start_time : chrono::DateTime<chrono::Utc>,
    pub server_version : String
} 

impl DaemonState {
    pub fn new(config: &Config, start_time : &chrono::DateTime<chrono::Utc>, server_version : &str) -> DaemonState {
        DaemonState {
            config : config.clone(),
            start_time : start_time.clone(),
            server_version : server_version.to_string()
        }
    }
} 

pub type Result<T, E = Error> = std::result::Result<T, E>;
