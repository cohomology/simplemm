use crate::{types, error};
use std::os::unix::net::UnixStream;
use snafu::ResultExt;
use std::io::Read;

pub fn stop_daemon(config: &types::Config) -> error::Result<()> {
    send_no_read(config, types::Action::Stop, None, None)?;
    let state = get_server_state(config);
    if let Err(_) = state {
        println!("Server stopped successfully");
    } else {
        println!("Could not stop server");
    }
    Ok(())
}

pub fn check_server_is_running(config: &types::Config) -> error::Result<(i64, types::DaemonState)> {
    let pid = get_server_pid(config)?;
    let state = get_server_state(&config)?;
    Ok((pid, state))
}

pub fn send_and_read<T: for<'de> serde::de::Deserialize<'de>>(config: &types::Config, 
                                                              action: types::Action, 
                                                              list_name: Option<String>,
                                                              data: Option<String>) -> error::Result<T> {
    let stream = send(config, action, list_name, data)?;
    let result: T = serde_json::from_reader(&stream).context(error::RequestParseError {})?;
    shutdown_socket(&stream, std::net::Shutdown::Both, config)?;
    Ok(result) 
}

pub fn send_no_read(config: &types::Config, action: types::Action, list_name: Option<String>, data: Option<String>) -> error::Result<()> {
    let stream = send(config, action, list_name, data)?;
    shutdown_socket(&stream, std::net::Shutdown::Both, config)?;
    Ok(())
}

fn shutdown_socket(stream: &UnixStream, shutdown : std::net::Shutdown, config: &types::Config) -> error::Result<()> {
    stream.shutdown(shutdown).context(
        error::SocketCloseError { 
            socket : &config.socket
        }
    )
}

fn get_server_pid(config: &types::Config) -> error::Result<i64> {
    let mut file = std::fs::File::open(&config.pid_file).context(
        error::PidFileReadError { 
            filename : &config.pid_file
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).context(
        error::PidFileReadError {
            filename : &config.pid_file
    })?;
    let pid : i64 = contents.trim().parse().context(
        error::PidFileParseError {
            filename : &config.pid_file
    })?;  
    Ok(pid)
}

fn get_server_state(config: &types::Config) -> error::Result<types::DaemonState> {
    send_and_read(config, types::Action::Alive, None, None)
}

fn send(config: &types::Config, action: types::Action, list_name: Option<String>, data: Option<String>) -> error::Result<UnixStream> {
    let stream = UnixStream::connect(&config.socket).context(
        error::SocketConnectError {
            socket : &config.socket
        }
    )?;
    
    let uid = users::get_current_uid();
    let user = users::get_user_by_uid(uid).map_or("<None>".to_string(), 
                                                  |user| user.name().to_string_lossy().to_string());
    let command = types::Command {
        action: action,
        originator: user,
        list_name: list_name,
        data: data,
    }; 
    serde_json::to_writer(&stream, &command).context(error::RequestSerializeError { })?;
    shutdown_socket(&stream, std::net::Shutdown::Write, config)?;
    Ok(stream)
}