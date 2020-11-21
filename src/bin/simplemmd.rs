#[macro_use]
extern crate log;
use snafu::{ErrorCompat, ResultExt};
use simplemm::types::{Error, Config, Result, DaemonizeError, SyslogError, SetLoggerError};
use syslog::{Facility, Formatter3164, BasicLogger};
use log::LevelFilter;
use daemonize::Daemonize;

fn error_abort(error : Error) -> ! {
    eprintln!("Error: {}", error); 
    if let Some(backtrace) = ErrorCompat::backtrace(&error) {
        eprintln!("{}", backtrace);
    }
    std::process::exit(-1);
}

fn daemonize(config : Config) -> Result<()> {
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

fn run() -> Result<()> {
    initialize_syslog()?;  
    let config = simplemm::config::read_config("simplemmd daemon")?;
    simplemm::database::check_database(&config)?;
    return daemonize(config);
}

fn main() {
    if let Err(e) = run() {
       error_abort(e)
    }
}
