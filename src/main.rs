mod config;
mod client;

use std::{env, process};

fn main() {
    let cfg = match config::read() {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("missing config, path: {}\n
                     rerun `lmz configure`", config::path());

            process::exit(1);
        },
    };

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
    }

    let cmd = args[1].as_str();
    match cmd {
        "configure" => {
            config::configure();
        },
        "status" => {
            let status = match client::get_status(cfg) {
                Ok(status) => status,
                Err(err) => err.to_string(),
            };

            println!("{}", status);
        },
        "on" | "off" => {
            let on = if cmd.eq("on") { true } else { false };
            let result = match client::put_status(cfg, on) {
                Ok(status) => status,
                Err(err) => err.to_string(),
            };

            println!("{}", result);
        },
        _ => {
            print_usage();
        }
    };

}

fn print_usage() {
    println!(
        "usage: lmz <command>\n\n\
        Command may be one of:\n\n\
        \tconfigure - create a config file\n\
        \tstatus - print the current status\n\
        \ton - set the status to 'on'\n\
        \toff - set the status to 'off'
        "
    );

    process::exit(1);
}

