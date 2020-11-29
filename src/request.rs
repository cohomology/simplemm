use crate::types::{Action, Command, DaemonState};
use crate::state::{stop_server, get_server_state};
use std::os::unix::net::UnixStream;

pub fn process_request(command: Command, stream: UnixStream) {
    match command.action {
       Action::Stop => stop_server(),
       Action::Alive => signal_alive(stream)
    }
}

fn signal_alive(stream: UnixStream) {
    let state = get_server_state();
    match state {
        Ok(ref state) => {
            send_signal_alive(stream, state);
        }
        Err(err) => {
            error!("Error when getting the server state: {:?}", err);
        }
    }
}

fn send_signal_alive(stream: UnixStream, state: &DaemonState) {
    if let Err(err) = serde_json::to_writer(&stream, state) {
        error!("Error serializing the server state: {:?}", err); 
    }
    let _ = stream.shutdown(std::net::Shutdown::Both);
}
