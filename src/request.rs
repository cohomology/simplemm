use crate::{types, state, error};
use std::os::unix::net::UnixStream;
use snafu::ResultExt;

pub fn process_request(command: types::Command, stream: UnixStream) {
    let result = match command.action {
       types::Action::Stop => Ok(state::stop_server()),
       types::Action::Alive => signal_alive(stream),
       types::Action::Subscribe => handle_subscribe(command),
    };
    if let Err(err) = result {
        log::error!("Error handling request: {}", err);
    }
}

fn signal_alive(stream: UnixStream) -> error::Result<()> {
    let state = state::get_server_state()?;
    send_signal_alive(stream, &state)
}

fn send_signal_alive(stream: UnixStream, state: &types::DaemonState) -> error::Result<()> {
    serde_json::to_writer(&stream, state).context(error::RequestSerializeError {})?;
    let _ = stream.shutdown(std::net::Shutdown::Both);
    Ok(())
}

fn handle_subscribe(command: types::Command) -> error::Result<()> {
    use mailparse::MailHeaderMap;
    if let Some(ref data) = command.data {
        let mail = mailparse::parse_mail(data.as_bytes()).context(error::MailParseError {})?;
        let from = mail.headers.get_first_value("From");
        log::info!("Got mail from: {:?}", from);
        let sender = mail.headers.get_first_value("Sender");
        log::info!("Sender: {:?}", sender);
    } else {
        log::warn!("Subscribe without any data");
    }
    Ok(())
}
