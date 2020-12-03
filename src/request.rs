use crate::{types, state};
use std::os::unix::net::UnixStream;

pub fn process_request(command: types::Command, stream: UnixStream) {
    match command.action {
       types::Action::Stop => state::stop_server(),
       types::Action::Alive => signal_alive(stream),
       types::Action::Subscribe => handle_subscribe(command),
    }
}

fn signal_alive(stream: UnixStream) {
    let state = state::get_server_state();
    match state {
        Ok(ref state) => {
            send_signal_alive(stream, state);
        }
        Err(err) => {
            log::error!("Error when getting the server state: {:?}", err);
        }
    }
}

fn send_signal_alive(stream: UnixStream, state: &types::DaemonState) {
    if let Err(err) = serde_json::to_writer(&stream, state) {
        log::error!("Error serializing the server state: {:?}", err); 
    }
    let _ = stream.shutdown(std::net::Shutdown::Both);
}

fn handle_subscribe(_command: types::Command) {
}
