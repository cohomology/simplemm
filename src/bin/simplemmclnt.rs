use simplemm::{error, types, client};
use snafu::ErrorCompat;

static PROGRAM: &'static str = "simplemmclnt";
static CLIENT_VERSION: &'static str = env!("CARGO_PKG_VERSION"); 

fn main() {
    if let Err(e) = run() {
       error_abort(e)
    }
} 

fn run() -> error::Result<()> {
    let (config, matches) = read_config()?;
    match matches.subcommand_name().unwrap() {
        "stop"      => action_stop(&config),
        "ping"      => action_ping(&config),
        "version"   => action_client_info(),
        "subscribe" => action_subscribe(&matches),        
        _         => Ok(())
    }
}

fn action_stop(config: &types::Config) -> error::Result<()> {
    let _ = client::check_server_is_running(&config)?;
    client::stop_daemon(config)
}

fn action_ping(config: &types::Config) -> error::Result<()> {
    let (pid, state) = client::check_server_is_running(&config)?;
    println!("simplemmd v{}, pid = {}, server_start_time: {}, uid = {}, gid = {}", state.server_version, 
        pid, state.start_time, state.config.uid, state.config.gid);
    Ok(())
}

fn action_client_info() -> error::Result<()> {
    println!("{} v{}", PROGRAM, CLIENT_VERSION);
    Ok(())
}

fn action_subscribe(matches: &clap::ArgMatches) -> error::Result<()> {
    let mailing_list = matches.subcommand_matches("subscribe")
                              .unwrap().value_of("list_name").unwrap();
    println!("{:?}", mailing_list); 
    Ok(())
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
        .subcommand(clap::SubCommand::with_name("version").about("Get client version"))
        .subcommand(clap::SubCommand::with_name("subscribe").about("Subscribe to mailing list")
                     .arg(clap::Arg::with_name("list_name").help("Name of the mailing list").required(true))); 
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