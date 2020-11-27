use crate::types::{Action, Command, QuitFn};

pub fn process_request(command: Command,
                       quit : QuitFn) {
    match command.action {
       Action::Stop => process_stop(quit),
       Action::Alive => signal_alive()
    }
}

fn process_stop(quit : QuitFn) {
    let func = quit.lock();
    match func {
        Ok(func) => func(),
        Err(err) => error!("Could not quit. Error in exit handler {:?}", err)
    }
}

fn signal_alive() {

}
