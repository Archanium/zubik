extern crate dotenv;
extern crate zubik;

fn main() {
    dotenv::dotenv().ok();
    zubik::app().launch();
}
