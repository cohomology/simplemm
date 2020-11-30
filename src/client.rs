use crate::types::*;
use std::os::unix::net::UnixStream;
use snafu::ResultExt;

pub fn send_and_read<T: for<'de> serde::de::Deserialize<'de>>(config: &Config, action: Action, data: Option<String>) -> Result<T> {
    let stream = send(config, action, data)?;
    let result: T = serde_json::from_reader(&stream).context(RequestParseError {})?;
    shutdown_socket(&stream, std::net::Shutdown::Both, config)?;
    Ok(result) 
}

pub fn send_no_read(config: &Config, action: Action, data: Option<String>) -> Result<()> {
    let stream = send(config, action, data)?;
    shutdown_socket(&stream, std::net::Shutdown::Both, config)?;
    Ok(())
}

fn send(config: &Config, action: Action, data: Option<String>) -> Result<UnixStream> {
    let stream = UnixStream::connect(&config.socket).context(
        SocketConnectError {
            socket : &config.socket
        }
    )?;
    
    let uid = users::get_current_uid();
    let user = users::get_user_by_uid(uid).map_or("<None>".to_string(), 
                                                  |user| user.name().to_string_lossy().to_string());
    let command = Command {
        action: action,
        originator: user,
        data: data,
    }; 
    serde_json::to_writer(&stream, &command).context(RequestSerializeError { })?;
    shutdown_socket(&stream, std::net::Shutdown::Write, config)?;
    Ok(stream)
}

pub fn shutdown_socket(stream: &UnixStream, shutdown : std::net::Shutdown, config: &Config) -> Result<()> {
    stream.shutdown(shutdown).context(
        SocketCloseError { 
            socket : &config.socket
        }
    )
}