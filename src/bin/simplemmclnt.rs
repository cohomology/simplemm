use simplemm::types::*;
use snafu::ErrorCompat;
use clap::{Arg,App};

static PROGRAM: &str = "simplemm client";

fn main() {
    if let Err(e) = run() {
       error_abort(e)
    }
} 

fn run() -> Result<()> {
    let (config, matches) = read_config()?;
    match matches.subcommand_name().unwrap() {
        "stop" => stop_daemon(),
        _      => ()
    };
    Ok(())
}

fn stop_daemon() {

}


fn error_abort(error : Error) -> ! {
    eprintln!("Error: {}", error); 
    if let Some(backtrace) = ErrorCompat::backtrace(&error) {
        eprintln!("{}", backtrace);
    }
    std::process::exit(-1)
}

fn parse_args<'a>() -> clap::ArgMatches<'a> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let app = App::new(PROGRAM).version(VERSION).author("by Cohomology, 2020")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("config").short("c")
             .long("config")
             .value_name("FILE")
             .help("configuration file")
             .takes_value(true))
        .subcommand(clap::SubCommand::with_name("stop").about("Stop simplemmd daemon"));
    let matches = app.get_matches();
    return matches;
}

fn read_config<'a>() -> Result<(Config, clap::ArgMatches<'a>)> {
    let arg_matches = parse_args();
    let config_file_name = arg_matches.value_of("config").unwrap_or("/etc/simplemm.conf");
    let config = simplemm::config::read_config(config_file_name)?;
    Ok((config, arg_matches))
}
