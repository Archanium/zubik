extern crate dotenv;
extern crate zubik;

use std::env;

fn main() {
    dotenv::dotenv().ok();
    zubik::app(zubik::Config::from_env()).launch();
}
