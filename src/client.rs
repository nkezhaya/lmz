use crate::config::Config;

use reqwest::blocking::RequestBuilder;
use reqwest::Result;
use reqwest::header;
use serde::Deserialize;
use std::collections::HashMap;
use std::process;

const STATUS_URL: &str = "https://gw.lamarzocco.io/v1/home/machines/LM035401/status";
const AUTH_URL: &str = "https://cms.lamarzocco.io/oauth/v2/token";

#[derive(Deserialize)]
struct StatusResponse {
    data: Machine
}

#[derive(Deserialize)]
struct Machine {
    #[serde(rename = "MACHINE_STATUS")]
    machine_status: String
}

pub fn get_status(cfg: Config) -> Result<String> {
    let client = reqwest::blocking::Client::new();

    println!("Fetching status...");
    let resp = put_headers(cfg, client.get(STATUS_URL))
        .send()?
        .json::<StatusResponse>();

    match resp {
        Ok(status) => return Ok(status.data.machine_status),
        Err(err) => return Err(err),
    };
}

pub fn put_status(cfg: Config, on: bool) -> Result<String> {
    let status = if on { "ON" } else { "STANDBY" };
    let mut body = HashMap::new();
    body.insert("status", status);

    println!("Updating status...");
    let client = reqwest::blocking::Client::new();

    put_headers(cfg, client.post(STATUS_URL))
        .json(&body)
        .send()?
        .text()
}

#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
}

pub fn get_auth_token(cfg: Config) -> Result<String> {
    let params = [
        ("client_id", cfg.client_id),
        ("client_secret", cfg.client_secret),
        ("grant_type", String::from("password")),
        ("password", cfg.password),
        ("username", cfg.username),
    ];

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(AUTH_URL)
        .form(&params)
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()?
        .json::<AccessToken>();

    match resp {
        Ok(token) => return Ok(token.access_token),
        Err(err) => {
            println!("error fetching auth token: {err:?}");
            process::exit(2);
        },
    };
}

fn put_headers(cfg: Config, builder: RequestBuilder) -> RequestBuilder {
    let auth_token = get_auth_token(cfg).unwrap();
    let auth_header = format!("Bearer {auth_token}");

    return builder
        .header(header::ACCEPT, "application/json")
        .header(header::AUTHORIZATION, auth_header)
        .header(header::CONTENT_TYPE, "application/json");
}
