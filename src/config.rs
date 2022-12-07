use confy;
use serde::{Serialize, Deserialize};
use std::io::{self, Write};

const APP_NAME: &str = "lmz";
const CONFIG_NAME: &str = "config";

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub access_token: String,
}

pub fn path() -> String {
    let path = confy::get_configuration_file_path(APP_NAME, CONFIG_NAME).expect("expected path");
    let path_string = String::from(path.to_str().unwrap());

    return path_string;
}

pub fn read() -> Result<Config, confy::ConfyError> {
    let cfg: Result<Config, confy::ConfyError> = confy::load(APP_NAME, CONFIG_NAME);

    return cfg;
}

pub fn configure() {
    let username = gets("username: ");
    let password = gets("password: ");
    let client_id = gets("client_id: ");
    let client_secret = gets("client_secret: ");

    let cfg: Config = Config {
        client_id: client_id.to_owned(),
        client_secret: client_secret.to_owned(),
        username: username.to_owned(),
        password: password.to_owned(),
        access_token: String::new(),
    };

    confy::store(APP_NAME, CONFIG_NAME, cfg).expect("error saving config");
}

fn gets(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("error flushing");

        let mut line = String::new();

        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                line = line.trim().to_owned();

                if line.len() == 0 {
                    continue;
                }

                return line.to_owned();
            },
            _ => continue,
        };
    }
}
