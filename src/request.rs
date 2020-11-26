use crate::types::{Action, Command, DaemonCommands};
use std::sync::mpsc::Sender;

pub fn process_request(sender : Sender<DaemonCommands>,
                       command: Command) {
    match command.action {
       Action::Stop => process_stop(sender)
    }
}

fn process_stop(sender : Sender<DaemonCommands>) {
    if let Err(err) = sender.send(DaemonCommands::StopDaemon) {
        warn!("Could not stop daemon: {:?}", err);
    }
}