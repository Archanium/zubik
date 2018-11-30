#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate tempfile;
#[cfg(test)]
extern crate mockito;


use rocket_contrib::json::Json;
use tempfile::NamedTempFile;
use std::process::Command;
use reqwest::RequestBuilder;
use std::env;
use rocket::State;

#[derive(Deserialize)]
struct ReleaseEvent {
    action: String,
    release: ReleaseData,
}

#[derive(Deserialize)]
struct ReleaseData {
    tag_name: String,
    assets: Vec<AssetData>,
}

#[derive(Deserialize)]
struct AssetData {
    url: String
}

struct App {
    config: Config
}

pub fn app(config: Config) -> rocket::Rocket {
    return rocket::ignite().manage(App { config }).mount("/", routes![release]);
}

fn add_oauth_token(request: RequestBuilder, conf: &Config) -> RequestBuilder {
    let ref oauth_token = conf.token;
    if oauth_token.is_empty() {
        return request;
    }
    return request.query(&[("access_token", oauth_token.as_str())]);
}

fn download_release(asset: &AssetData, conf: &Config) -> String {
    let ref release_script = conf.script;
    if release_script.is_empty() {
        return String::from("No release script defined");
    }

    let url = asset.url.to_string();
    let client = reqwest::Client::new();

    let request = client.get(url.as_str());
    let request = request.header("Accept", "application/octet-stream");
    let request = add_oauth_token(request, conf);


    let mut output_file: NamedTempFile = NamedTempFile::new().expect("Could not create temp file");
    let mut response = request.send().expect("Error sending request to");
    response.copy_to(&mut output_file).expect("Could not write file");

    let output = Command::new(release_script).arg(output_file.path()).output().expect("Error opening script");
    let res;
    if !output.status.success() {
        res = String::from_utf8(output.stderr);
    } else {
        res = String::from_utf8(output.stdout);
    }
    return res.expect("UTF8 error");
}

#[post("/webhook", data = "<payload>")]
fn release<'a>(payload: Json<ReleaseEvent>, app_config: State<App>) -> String {
    let assets = &payload.release.assets;
    let output;
    let response_parts = match Vec::len(assets) {
        1 => {
            let ref asset = assets.first().expect("No asset gotten!");
            output = download_release(asset, &app_config.config).to_string();
            output.as_str()
        }
        _ => "Not handled"
    };

    response_parts.to_string()
}


#[cfg(test)]
mod tests;

pub struct Config {
    pub token: String,
    pub script: String,
}

impl Config {
    pub fn new(token: String, script: String) -> Config {
        Config { token, script }
    }
    pub fn from_env() -> Config {
        let script = env::var("RELEASE_SCRIPT").unwrap();
        let mut token = "".to_string();
        let env_token = env::var("GITHUB_OAUTH_TOKEN");
        if env_token.is_ok() {
            token.push_str(env_token.unwrap().as_str())
        }

        Config { token, script }
    }
}