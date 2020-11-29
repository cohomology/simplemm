use crate::types::{Action, Command};
use crate::state::stop_server;

pub fn process_request(command: Command) {
    match command.action {
       Action::Stop => stop_server(),
       Action::Alive => signal_alive()
    }
}

fn signal_alive() {

}
