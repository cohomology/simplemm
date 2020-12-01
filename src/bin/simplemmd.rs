use simplemm::types::*;
use simplemm::state::get_server_version;

use snafu::{ErrorCompat, ResultExt};
use syslog::{Facility, Formatter3164, BasicLogger};
use log::LevelFilter;
use daemonize::Daemonize;
use std::thread;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::io::BufReader;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use clap::{Arg,App};

static PROGRAM: &str = "simplemmd daemon";

fn main() {
    if let Err(e) = run() {
       error_abort(e)
    }
} 

fn run() -> Result<()> {
    initialize_syslog()?;  
    let config = read_config()?;
    pre_daemonize_checks(&config)?;
    let socket = bind_to_socket(&config)?;
    daemonize(&config)?;
    Ok(handle_requests(socket))
} 

fn read_config<'a>() -> Result<Config> {
    let arg_matches = parse_args();
    let config_file_name = arg_matches.value_of("config").unwrap_or("/etc/simplemm.conf");
    let config = simplemm::config::read_config(config_file_name)?;
    Ok(config)
}

fn parse_args<'a>() -> clap::ArgMatches<'a> {
  let matches = App::new(PROGRAM).version(get_server_version()).author("by Cohomology, 2020")
                             .arg(Arg::with_name("config").short("c")
                                                          .long("config")
                                                          .value_name("FILE")
                                                          .help("configuration file")
                                                          .takes_value(true))
                             .get_matches();
  return matches;
}

fn pre_daemonize_checks(config :&Config) -> Result<()> {
    simplemm::database::check_database(&config)?; 
    simplemm::file::check_working_dir(&config)?; 
    simplemm::file::check_pid_file(&config)?;

    Ok(())
}

fn daemonize(config : &Config) -> Result<()> {
     let daemonize = Daemonize::new()
        .pid_file(&config.pid_file) 
        .chown_pid_file(true)      
        .working_directory(&config.working_dir)
        .user(config.uid)
        .group(config.gid)      
        .umask(0o777);

    daemonize.start().context(DaemonizeError {})?;
    simplemm::state::start_server(&config)?;
    Ok(())
}

fn bind_to_socket(config : &Config) -> Result<UnixListener> {
    let path = Path::new(&config.socket);
    let _ = std::fs::remove_file(&path);
    let listener = UnixListener::bind(&path).context(SocketBindError { 
            path : path.to_string_lossy().to_string() 
    })?;
    set_socket_permissions(&config.socket)?;
    Ok(listener)
}

fn handle_requests(listener: UnixListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(err) => {
                log::error!("Error: {}", err);
                break;
            }
        }
    } 
}

fn handle_client(stream: UnixStream) {
    let reader = BufReader::new(&stream);
    let command: Result<Command> = 
        serde_json::from_reader(reader).context(RequestParseError {});
    match command {
        Ok(command) => simplemm::request::process_request(command, stream),
        Err(err) => log::warn!("Could not parse request: {:?}", err)
    }
}

fn initialize_syslog() -> Result<()> {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "simplemmd".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).context(SyslogError {})?;
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Info)).context(SetLoggerError {})?;
    Ok(())
}

fn error_abort(error : Error) -> ! {
    log::error!("Error: {}", error);
    eprintln!("Error: {}", error); 
    if let Some(backtrace) = ErrorCompat::backtrace(&error) {
        log::error!("{}", backtrace);
        eprintln!("{}", backtrace);
    }
    std::process::exit(-1)
}

fn set_socket_permissions(socket : &str) -> Result<()> {
    fs::set_permissions(socket, fs::Permissions::from_mode(0o777))
         .context(SocketPermissionError { socket })
}
