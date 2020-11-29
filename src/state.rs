use crate::types::*; 
use crate::file::delete_file;
use chrono::{DateTime, Utc};
use std::sync::RwLock;
use snafu::ResultExt;

struct DaemonState {
    pub config : Config,
    pub start_time : DateTime<Utc>
}

impl DaemonState {
    fn new(config: &Config, start_time : &DateTime<Utc>) -> DaemonState {
        DaemonState {
            config : config.clone(),
            start_time : start_time.clone()
        }
    }
}

lazy_static! {
    static ref STATE: RwLock<Option<DaemonState>> = RwLock::new(None);
} 

pub fn start_server(config : &Config) -> Result<()> {
    log_start(&config);
    let now = Utc::now();
    let mut state = STATE.write().map_err(
        |err| Box::new(err) as Box<dyn std::error::Error>)
        .context(ServerStateError {})?;
    *state = Some(DaemonState::new(config, &now));
    set_exit_handler()?;
    Ok(())
}

pub fn stop_server() {
    let state = STATE.write();
    if let Ok(mut state) = state {
        if let Some(ref state) = *state {
            let config = state.config.clone();
            delete_file(&config.socket);
            delete_file(&config.pid_file); 
            log_end(&config);
        }
        *state = None;
    }
    std::process::exit(-1);
}

fn set_exit_handler() -> Result<()> {
    ctrlc::set_handler(move || {
        stop_server()
    }).context(ExitHandlerError {})?;
    Ok(())
} 

fn log_start(config: &Config) {
    info!("simplemmd started, uid = {}, gid = {}", config.uid, config.gid);
}

fn log_end(config: &Config) {
    info!("simplemmd stopped, uid = {}, gid = {}", config.uid, config.gid);
} 
