mod config;
mod client;

use clap;
use std::process;

fn main() {
    let cfg = match config::read() {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("missing config, path: {}\n
                     rerun `lmz configure`", config::path());

            process::exit(1);
        },
    };

    let cmd = clap::Command::new("lmz")
        .bin_name("lmz")
        .subcommand_required(true)
        .subcommand(clap::command!("configure").about("creates a config file"))
        .subcommand(clap::command!("status").about("prints the current status"))
        .subcommand(clap::command!("on").about("turns it on"))
        .subcommand(clap::command!("off").about("turns it off"));

    match cmd.get_matches().subcommand() {
        Some(("configure", _)) => config::configure(),
        Some(("status", _)) => {
            let status = match client::get_status(cfg) {
                Ok(status) => status,
                Err(err) => err.to_string(),
            };

            println!("{}", status);
        },
        Some(("on", _)) => put_status(cfg, true),
        Some(("off", _)) => put_status(cfg, false),
        _ => unreachable!(),

    };
}

fn put_status(cfg: config::Config, on: bool) {
    let result = match client::put_status(cfg, on) {
        Ok(status) => status,
        Err(err) => err.to_string(),
    };

    println!("{}", result);
}
