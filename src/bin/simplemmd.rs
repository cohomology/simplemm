#[macro_use]
extern crate log;

use snafu::{ErrorCompat, ResultExt};
use simplemm::types::*;
use syslog::{Facility, Formatter3164, BasicLogger};
use log::LevelFilter;
use daemonize::Daemonize;
use faccess::{AccessMode, PathExt};
use std::path::{Path, PathBuf};
use std::thread;
use std::os::unix::net::{UnixStream,UnixListener};

fn main() {
    if let Err(e) = run() {
       error_abort(e)
    }
} 

fn run() -> Result<()> {
    initialize_syslog()?;  
    let config = simplemm::config::read_config("simplemmd daemon")?;
    pre_daemonize_checks(&config)?;
    daemonize(&config)?;
    bind_to_socket(&config)
} 

fn pre_daemonize_checks(config :&Config) -> Result<()> {
    simplemm::database::check_database(&config)?; 
    check_working_dir(&config)?; 
    check_pid_file(&config)?;
    Ok(())
}

fn check_working_dir(config: &Config) -> Result<()> {
    let path = Path::new(&config.working_dir);
    check_writeable(&path)?;
    Ok(())
}

fn check_pid_file(config: &Config) -> Result<()> {
    let path = Path::new(&config.pid_file);
    check_writeable_file(&path)?;
    Ok(())
}

fn check_writeable_file(path: &Path) -> Result<()> {
    if path.exists() {
        check_is_file(path)?;
        check_writeable(path)?;
    } else {
        check_parent_dir_writeable(path)?;
    }
    Ok(())
}

fn check_parent_dir_writeable(path :&Path) -> Result<()> {
    let mut path_buf = PathBuf::new();
    path_buf.push(path);
    path_buf.pop();
    check_writeable(path_buf.as_path())?;
    Ok(())
}

fn check_is_file(path :&Path) -> Result<()> {
    let metadata = std::fs::metadata(path).context(PathMetadataError { 
        path : path.to_string_lossy().to_string()
    })?;
    if ! metadata.is_file() {
        return Err(Error::PathNoFile { 
            path : path.to_string_lossy().to_string()
        });
    }
    Ok(())
}

fn check_writeable(path : &Path) -> Result<()> {
    if ! path.access(AccessMode::READ | 
                     AccessMode::WRITE).is_ok() {
      return Err(Error::CouldNotWriteToFileOrDirectory { 
          path : path.to_string_lossy().to_string() }
      ); 
    }
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
    log_start(&config);
    Ok(())
}

fn bind_to_socket(config : &Config) -> Result<()> {
    let listener = UnixListener::bind(&config.socket).context(
        SocketBindError { path : &config.socket }
    )?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                error!("Error: {}", err);
                break;
            }
        }
    }
    Ok(())
}

fn handle_client(_stream: UnixStream) {
}

fn initialize_syslog() -> Result<()> {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "myprogram".into(),
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

fn error_abort(error : Error) -> ! {
    error!("Error: {}", error);
    eprintln!("Error: {}", error); 
    if let Some(backtrace) = ErrorCompat::backtrace(&error) {
        error!("{}", backtrace);
        eprintln!("{}", backtrace);
    }
    std::process::exit(-1)
}