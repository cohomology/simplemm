use simplemm::{error, types, client};
use snafu::ErrorCompat;

static PROGRAM: &'static str = "simplemm client";
static CLIENT_VERSION: &'static str = env!("CARGO_PKG_VERSION"); 

fn main() {
    if let Err(e) = run() {
       error_abort(e)
    }
} 

fn run() -> error::Result<()> {
    let (config, matches) = read_config()?;
    let (pid, state) = client::check_server_is_running(&config)?;
    match matches.subcommand_name().unwrap() {
        "stop" => client::stop_daemon(&config),
        "ping" => Ok(print_server_state(pid, &state)),
        "version" => Ok(print_client_info()),
        _      => Ok(())
    }
}

fn print_server_state(pid: i64, state: &types::DaemonState) {
    println!("Server is running, pid = {}, server_start_time: {}", pid, state.start_time);
}

fn print_client_info() {
    println!("{}, v{}", PROGRAM, CLIENT_VERSION);

}

fn parse_args<'a>() -> clap::ArgMatches<'a> {
    let app = clap::App::new(PROGRAM).version(CLIENT_VERSION).author("by Cohomology, 2020")
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

fn read_config<'a>() -> error::Result<(types::Config, clap::ArgMatches<'a>)> {
    let arg_matches = parse_args();
    let config_file_name = arg_matches.value_of("config").unwrap_or("/etc/simplemm.conf");
    let config = simplemm::config::read_config(config_file_name)?;
    Ok((config, arg_matches))
}

fn error_abort(error : error::Error) -> ! {
    eprintln!("Error: {}", error); 
    if let Some(backtrace) = ErrorCompat::backtrace(&error) {
        eprintln!("{}", backtrace);
    }
    std::process::exit(-1)
}