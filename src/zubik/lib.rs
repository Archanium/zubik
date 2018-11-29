#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code)]
#[macro_use]
extern crate rocket;

extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests;

use rocket_contrib::json::Json;

pub fn app() -> rocket::Rocket {
    return rocket::ignite().mount("/", routes![release]);
}

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

#[post("/webhook", data = "<payload>")]
fn release(payload: Json<ReleaseEvent>) -> String {
    format!("Releasing: {}", payload.release.tag_name)
}
