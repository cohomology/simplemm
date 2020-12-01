use serde::{Serialize, Deserialize};

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