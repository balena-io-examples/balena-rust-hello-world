use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("name").index(1).required(true))
        .get_matches();

    // Infinite loop - otherwise the application will quit and the container
    // will be launched again and again and your logs will be flooded with
    // the "Hello, {}!" messages.
    loop {
        println!("Hello, {}!", matches.value_of("name").unwrap());
        std::thread::sleep(std::time::Duration::new(10, 0));
    }
}
