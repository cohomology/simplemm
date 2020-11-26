#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use simplemm::types::*;

use snafu::{ErrorCompat, ResultExt};
use syslog::{Facility, Formatter3164, BasicLogger};
use log::LevelFilter;
use daemonize::Daemonize;
use std::thread;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::io::BufReader;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::Mutex;
use clap::{Arg,ArgMatches,App};

static PROGRAM: &str = "simplemmd daemon";

lazy_static! {
    static ref CONFIG_FILE: Mutex<String> = Mutex::new(String::new());
}

fn main() {
    if let Err(e) = run() {
       error_abort(e)
    }
} 

fn run() -> Result<()> {
    initialize_syslog()?;  
    let config = read_config()?;
    pre_daemonize_checks(&config)?;
    daemonize(&config)?;
    bind_to_socket(&config)
} 

fn read_config<'a>() -> Result<Config> {
    let arg_matches = parse_args();
    let config_file_name = arg_matches.value_of("config").unwrap_or("/etc/simplemm.conf");
    let config = simplemm::config::read_config(config_file_name);
    let config_file = CONFIG_FILE.lock()?;
    config_file = config_file_name;
    Ok(config)
}

fn parse_args<'a>() -> clap::ArgMatches<'a> {
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");
  let matches = clap::App::new(PROGRAM).version(VERSION).author("by Cohomology, 2020")
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
    set_exit_handler()?;
    log_start(&config);
    Ok(())
}

fn bind_to_socket(config : &Config) -> Result<()> {
    type MpscTuple = (Sender<DaemonCommands>, Receiver<DaemonCommands>);
    let path = Path::new(&config.socket);
    let _ = std::fs::remove_file(&path);
    let listener = UnixListener::bind(&path).context(SocketBindError { 
            path : path.to_string_lossy().to_string() 
    })?;

    let (tx, rx) : MpscTuple = channel();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let sender = tx.clone();
                thread::spawn(move || handle_client(sender, stream));
            }
            Err(err) => {
                error!("Error: {}", err);
                break;
            }
        }
        if let Ok(DaemonCommands::StopDaemon) = rx.try_recv() {
            break;
        }
    }
    Ok(())
}

fn handle_client(sender : Sender<DaemonCommands>, stream: UnixStream) {
    let reader = BufReader::new(stream);
    let command: Result<Command> = 
        serde_json::from_reader(reader).context(RequestParseError {});
    match command {
        Ok(command) => simplemm::request::process_request(sender, command),
        Err(err) => warn!("Could not parse request: {:?}", err)
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

fn log_start(config: &Config) {
    info!("simplemmd started, uid = {}, gid = {}", config.uid, config.gid);
}

fn log_end(config: &Result<Config>) {
    if let Ok(config) = config {
        info!("simplemmd stopped, uid = {}, gid = {}", config.uid, config.gid);
    } else {
        info!("simplemmd stopped");
    }
}

fn error_abort(error : Error) -> ! {
    error!("Error: {}", error);
    eprintln!("Error: {}", error); 
    if let Some(backtrace) = ErrorCompat::backtrace(&error) {
        error!("{}", backtrace);
        eprintln!("{}", backtrace);
    }
    std::process::exit(-1)
}

fn set_exit_handler() -> Result<()> {
    ctrlc::set_handler(move || {
        exit_handler()
    }).context(ExitHandlerError {})?;
    Ok(())
}

fn exit_handler() -> ! {
    let config = read_config();
    if let Ok(ref config) = config {
        delete_file(&config.socket);
        delete_file(&config.pid_file);
    }
    log_end(&config);
    std::process::exit(-1)
}

fn delete_file(file_path : &str) {
    let path = Path::new(&file_path);
    let _ = std::fs::remove_file(&path);
}