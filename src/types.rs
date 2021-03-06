use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub db_url: String,
    pub uid: u32,
    pub gid: u32,
    pub pid_file: String,
    pub working_dir: String,
    pub socket: String,
}

#[derive(Serialize, Deserialize)]
pub enum Action {
    Stop,
    Alive,
    Subscribe,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub action: Action,
    pub originator: String,
    pub list_name: Option<String>,
    pub data: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DaemonState {
    pub config: Config,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub server_version: String,
}

pub struct Subscription {
    pub email: String,
    pub uuid: String,
}

impl DaemonState {
    pub fn new(
        config: &Config,
        start_time: &chrono::DateTime<chrono::Utc>,
        server_version: &str,
    ) -> DaemonState {
        DaemonState {
            config: config.clone(),
            start_time: *start_time,
            server_version: server_version.to_string(),
        }
    }
}
