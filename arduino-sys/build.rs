extern crate pkg_config;

fn main() {
    match pkg_config::find_library("libarduino").unwrap()
}
