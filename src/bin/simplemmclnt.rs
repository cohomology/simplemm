use simplemm::types::*;
use snafu::{ErrorCompat, ResultExt};
use std::io::Read;

static PROGRAM: &'static str = "simplemm client";
static VERSION: &'static str = env!("CARGO_PKG_VERSION"); 

fn main() {
    if let Err(e) = run() {
       error_abort(e)
    }
} 

fn run() -> Result<()> {
    let (config, matches) = read_config()?;
    let (pid, state) = check_server_is_running(&config)?;
    match matches.subcommand_name().unwrap() {
        "stop" => stop_daemon(&config),
        "ping" => Ok(print_server_state(pid, &state)),
        "version" => Ok(print_client_info()),
        _      => Ok(())
    }
}

fn error_abort(error : Error) -> ! {
    eprintln!("Error: {}", error); 
    if let Some(backtrace) = ErrorCompat::backtrace(&error) {
        eprintln!("{}", backtrace);
    }
    std::process::exit(-1)
}

fn parse_args<'a>() -> clap::ArgMatches<'a> {
    let app = clap::App::new(PROGRAM).version(VERSION).author("by Cohomology, 2020")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .arg(clap::Arg::with_name("config").short("c")
             .long("config")
             .value_name("FILE")
             .help("configuration file")
             .takes_value(true))
        .subcommand(clap::SubCommand::with_name("stop").about("Stop simplemmd daemon"))
        .subcommand(clap::SubCommand::with_name("ping").about("Get server status"))
        .subcommand(clap::SubCommand::with_name("version").about("Get client version")); 
    let matches = app.get_matches();
    return matches;
}

fn read_config<'a>() -> Result<(Config, clap::ArgMatches<'a>)> {
    let arg_matches = parse_args();
    let config_file_name = arg_matches.value_of("config").unwrap_or("/etc/simplemm.conf");
    let config = simplemm::config::read_config(config_file_name)?;
    Ok((config, arg_matches))
}

fn check_pid_file_exists(config: &Config) -> Result<i64> {
    let mut file = std::fs::File::open(&config.pid_file).context(
        PidFileReadError { 
            filename : &config.pid_file
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).context(
        PidFileReadError {
            filename : &config.pid_file
    })?;
    let pid : i64 = contents.trim().parse().context(
        PidFileParseError {
            filename : &config.pid_file
    })?;  
    Ok(pid)
}

fn check_server_is_running(config: &Config) -> Result<(i64, DaemonState)> {
    let pid = check_pid_file_exists(config)?;
    let state = get_server_state(&config)?;
    Ok((pid, state))
}

fn get_server_state(config: &Config) -> Result<DaemonState> {
    simplemm::client::send_and_read(config, Action::Alive, None)
}

fn stop_daemon(config: &Config) -> Result<()> {
    simplemm::client::send_no_read(config, Action::Stop, None)?;
    let state = get_server_state(config);
    if let Err(_) = state {
        println!("Server stopped successfully");
    } else {
        println!("Could not stop server");
    }
    Ok(())
}


fn print_server_state(pid: i64, state: &DaemonState) {
    println!("Server is running, pid = {}, server_start_time: {}", pid, state.start_time);
}

fn print_client_info() {
    println!("{}, v{}", PROGRAM, VERSION);

}
