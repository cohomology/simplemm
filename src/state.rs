use crate::error::*;
use crate::types::*; 
use crate::file;
use snafu::ResultExt;

lazy_static::lazy_static! {
    static ref STATE: std::sync::RwLock<Option<DaemonState>> = std::sync::RwLock::new(None);
} 

pub fn get_server_version() -> &'static str {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    return VERSION;
}

pub fn start_server(config : &Config) -> Result<()> {
    log_start(&config);
    let now = chrono::Utc::now();
    let mut state = STATE.write().map_err(
        |err| Box::new(err) as Box<dyn std::error::Error>)
        .context(ServerStateError {})?;
    *state = Some(DaemonState::new(config, &now, get_server_version()));
    set_exit_handler()?;
    Ok(())
}

pub fn stop_server() {
    let state = STATE.write();
    if let Ok(mut state) = state {
        if let Some(ref state) = *state {
            let config = state.config.clone();
            file::delete_file(&config.socket);
            file::delete_file(&config.pid_file); 
            log_end(&config);
        }
        *state = None;
    }
    std::process::exit(-1);
}

pub fn get_server_state() -> Result<DaemonState> {
    let lock_state = STATE.read().map_err(
        |err| Box::new(err) as Box<dyn std::error::Error>)
        .context(ServerStateError {})?;
    let state = (*lock_state).as_ref().unwrap();
    Ok(state.clone())
}

fn set_exit_handler() -> Result<()> {
    ctrlc::set_handler(move || {
        stop_server()
    }).context(ExitHandlerError {})?;
    Ok(())
} 

fn log_start(config: &Config) {
    log::info!("simplemmd started, uid = {}, gid = {}", config.uid, config.gid);
}

fn log_end(config: &Config) {
    log::info!("simplemmd stopped, uid = {}, gid = {}", config.uid, config.gid);
} 
