use clap::{App, Arg};

fn main() {
    let version = env!("CARGO_PKG_VERSION");

    let matches = App::new("Key/Value store")
        .version(version)
        .arg(Arg::with_name("get"))
        .arg(Arg::with_name("set"))
        .arg(Arg::with_name("rm"))
        .get_matches();

    if !matches.is_present("V") {
        panic!("unimplemented");
    }
}
