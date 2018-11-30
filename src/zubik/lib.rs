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

pub fn app() -> rocket::Rocket {
    return rocket::ignite().mount("/", routes![release]);
}

fn download_release(asset: &AssetData, tag_name: &String) -> String {
    format!("Releasing: {}", tag_name);
    let url = asset.url.to_string();
    let mut output_file: NamedTempFile = NamedTempFile::new().expect("Could not create temp file");
    reqwest::get(url.as_str()).expect("Error sending request to")
        .copy_to(&mut output_file).expect("Could not write file");
    let release_script = dotenv::var("RELEASE_SCRIPT");
    if release_script.is_err() {
        return String::from("No release script defined");
    }
    let release_script = release_script.unwrap();

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
fn release<'a>(payload: Json<ReleaseEvent>) -> String {
    let assets = &payload.release.assets;
    let output;
    let response_parts = match Vec::len(assets) {
        1 => {
            let ref asset = assets.first().expect("No asset gotten!");
            let ref tag_name = payload.release.tag_name;
            output = download_release(asset, tag_name).to_string();
               output.as_str()

        }
        _ => "Not handled"
    };

    response_parts.to_string()
}


#[cfg(test)]
mod tests;
