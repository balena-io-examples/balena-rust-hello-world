#[macro_use]
extern crate clap;

use clap::{App, Arg};

/// Check that the command line `NAME` argument isn't empty.
///
/// # Arguments
///
/// * `value` - command line `NAME` argument value
fn validate_name(value: String) -> Result<(), String> {
    match value.trim().is_empty() {
        true => Err("I can't say hello to empty NAME".to_string()),
        false => Ok(()),
    }
}

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("NAME")
                .takes_value(true)
                .required(false)
                .default_value("world")
                .validator(validate_name),
        )
        .get_matches();

    println!("Hello, {}!", matches.value_of("NAME").unwrap().trim());
}
