use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let is_ignore_case = env::var("IGNORE_CASE").is_ok();
    let config = Config::from_iter(env::args());
    if let Err(err) = minigrep::run(config, is_ignore_case) {
        eprintln!("error: {err}");
        process::exit(1);
    }
}