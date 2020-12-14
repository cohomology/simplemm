use simplemm::{error, state, types};

use snafu::{ErrorCompat, ResultExt};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};

static PROGRAM: &str = "simplemmd daemon";

fn main() {
    if let Err(e) = run() {
        error_abort(e)
    }
}

fn run() -> error::Result<()> {
    initialize_syslog()?;
    let config = read_config()?;
    pre_daemonize_checks(&config)?;
    let socket = bind_to_socket(&config)?;
    daemonize(&config)?;
    handle_requests(socket);
    Ok(())
}

fn read_config() -> error::Result<types::Config> {
    let arg_matches = parse_args();
    let config_file_name = arg_matches
        .value_of("config")
        .unwrap_or("/etc/simplemm.conf");
    let config = simplemm::config::read_config(config_file_name)?;
    Ok(config)
}

fn parse_args<'a>() -> clap::ArgMatches<'a> {
    let matches = clap::App::new(PROGRAM)
        .version(state::get_server_version())
        .author("by Cohomology, 2020")
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("configuration file")
                .takes_value(true),
        )
        .get_matches();
    matches
}

fn pre_daemonize_checks(config: &types::Config) -> error::Result<()> {
    simplemm::database::check_database(&config)?;
    simplemm::file::check_working_dir(&config)?;
    simplemm::file::check_pid_file(&config)?;
    Ok(())
}

fn daemonize(config: &types::Config) -> error::Result<()> {
    let daemonize = daemonize::Daemonize::new()
        .pid_file(&config.pid_file)
        .chown_pid_file(true)
        .working_directory(&config.working_dir)
        .user(config.uid)
        .group(config.gid)
        .umask(0o777);

    daemonize.start().context(error::DaemonizeError {})?;
    simplemm::state::start_server(&config)?;
    Ok(())
}

fn bind_to_socket(config: &types::Config) -> error::Result<UnixListener> {
    let path = std::path::Path::new(&config.socket);
    let _ = std::fs::remove_file(&path);
    let listener = UnixListener::bind(&path).context(error::SocketBindError {
        path: path.to_string_lossy().to_string(),
    })?;
    set_socket_permissions(&config.socket)?;
    Ok(listener)
}

fn handle_requests(listener: UnixListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || handle_client(stream));
            }
            Err(err) => {
                log::error!("Error: {}", err);
                break;
            }
        }
    }
}

fn handle_client(stream: UnixStream) {
    let reader = std::io::BufReader::new(&stream);
    let command: error::Result<types::Command> =
        serde_json::from_reader(reader).context(error::RequestParseError {});
    match command {
        Ok(command) => simplemm::request::process_request(command, stream),
        Err(err) => log::warn!("Could not parse request: {:?}", err),
    }
}

fn initialize_syslog() -> error::Result<()> {
    let formatter = syslog::Formatter3164 {
        facility: syslog::Facility::LOG_USER,
        hostname: None,
        process: "simplemmd".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).context(error::SyslogError {})?;
    log::set_boxed_logger(Box::new(syslog::BasicLogger::new(logger)))
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .context(error::SetLoggerError {})?;
    Ok(())
}

fn error_abort(error: error::Error) -> ! {
    log::error!("Error: {}", error);
    eprintln!("Error: {}", error);
    if let Some(backtrace) = ErrorCompat::backtrace(&error) {
        log::error!("{}", backtrace);
        eprintln!("{}", backtrace);
    }
    std::process::exit(-1)
}

fn set_socket_permissions(socket: &str) -> error::Result<()> {
    std::fs::set_permissions(socket, std::fs::Permissions::from_mode(0o777))
        .context(error::SocketPermissionError { socket })
}
