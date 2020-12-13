use crate::{types, state, error, parse_mail, database};
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
    let data = command.data.ok_or(error::Error::SubscriptionRequestWithoutData)?;
    let mail = mailparse::parse_mail(data.as_bytes()).context(error::MailParseError {})?;
    let from = mail.headers.get_first_value("From").ok_or(
        error::Error::EmptyOrMissingHeader {
            header : "FROM",
            request : data.clone()
    })?;
    let addresses = parse_mail::get_addresses_in_from_header(&from);
    if addresses.is_empty() {
        return Err(error::Error::CouldNotParseHeader {
            header: "FROM",
            request : data.clone()
        });
    }
    let list_name = command.list_name.ok_or(
        error::Error::RequestWithoutListName {
            request_type : "SUBSCRIBE",
            request : data.clone()
    })?; 
    let subscriptions = addresses.into_iter().map(|address| types::Subscription { 
        email: address,
        uuid: uuid::Uuid::new_v4().to_string(),
    }).collect();
    database::insert_subscriptions(&list_name, subscriptions, &data, send_mail_for_subscription)?;
    Ok(())
}

fn send_mail_for_subscription(_subscription: &types::Subscription) -> error::Result<()> {
    Ok(())
}