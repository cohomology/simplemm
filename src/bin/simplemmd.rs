use snafu::ErrorCompat;

fn main() {
    let config = simplemm::config::read_config("simplemmd daemon");
    if let Err(e) = config {
      eprintln!("Error: {}", e); 
      if let Some(backtrace) = ErrorCompat::backtrace(&e) {
          println!("{}", backtrace);
      }
    }
}
