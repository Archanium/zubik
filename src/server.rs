extern crate dotenv;
extern crate zubik;

use std::io::{self, Write};

fn main() {
    dotenv::dotenv().ok();
    let config = zubik::Config::from_env();
    io::stdout()
        .write(
            format!(
                "Running with config: token: {} script: {}",
                config.token.as_str(),
                config.script.as_str()
            )
            .as_bytes(),
        )
        .expect("Invalid config");

    zubik::app(config).launch();
}
