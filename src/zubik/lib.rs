#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use rocket_contrib::json::Json;
use std::fs::File;

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

fn download_release(asset: &AssetData, tag_name: String) {
    format!("Releasing: {}", tag_name);
    let temp_path = std::env::temp_dir();
    let temp_file = temp_path.join("/release");
    let mut output_file = File::create(temp_file.as_path()).expect("Unable to create temp file");
    reqwest::get(asset.url.as_str()).expect("Error sending request to")
        .copy_to(&mut output_file).expect("Could not write file");

    // TODO: Run a script that takes the temp file as an argument.
}

#[post("/webhook", data = "<payload>")]
fn release(payload: Json<ReleaseEvent>) -> String {
    let assets = &payload.release.assets;

    match Vec::len(assets) {
        1 => {
            let asset = assets.first().expect("No asset gotten!");
            download_release(asset, payload.release.tag_name.clone());
            "Handled"
        }
        _ => "Not handled"
    }.to_string()
}


#[cfg(test)]
mod tests;
