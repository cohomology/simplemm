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
        source: mysql::Error
    },
    #[snafu(display("Could not prepare statement \"{}\": \"{}\"", statement, source))]
    DbPrepareError {
        statement: &'static str,
        source: mysql::Error
    },
    #[snafu(display("Could not execute statement \"{}\": \"{}\"", statement, source))]
    DbExecuteError {
        statement: &'static str,
        source: mysql::Error
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
    #[snafu(display("Could not read data from stdin: {}", source))]
    ReadStdinError {
        source : std::io::Error
    },       
    #[snafu(display("Could not parse mail: {}", source))]
    MailParseError {
        source : mailparse::MailParseError
    },   
    #[snafu(display("Subscription request without data"))]
    SubscriptionRequestWithoutData,
    #[snafu(display("Empty or missing {} header in request \"{:?}\"", header, request))]
    EmptyOrMissingHeader {
        header : &'static str,
        request: String
    },
    #[snafu(display("Could not parse {} header in request \"{:?}\"", header, request))]
    CouldNotParseHeader {
        header : &'static str,
        request: String
    },
    #[snafu(display("Request {} without list name: \"{:?}\"", request_type, request))]
    RequestWithoutListName {
        request_type : &'static str,
        request: String
    },
    #[snafu(display("Mailing list {} does not exist in the database", list_name))]
    DbMailingListDoesNotExist {
        list_name : String
    },
}
 
pub type Result<T, E = Error> = std::result::Result<T, E>;
