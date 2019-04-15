#[macro_use]
extern crate clap;
extern crate node_extra_exporter;

use clap::{App, Arg};
use node_extra_exporter::server;

fn main() {
    let matches = App::new("node_extra_exporter")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .args(&[
            Arg::with_name("address")
                .default_value("0.0.0.0:9001")
                .short("a")
                .long("address")
                .help("IP address to listen for Prometheus requests"),
            Arg::with_name("procfs")
                .default_value("/proc")
                .short("p")
                .long("procfs")
                .help("Location where `procfs` is mounted to"),
        ])
        .get_matches();

    server::serve(
        &value_t!(matches, "address", String).unwrap(),
        value_t!(matches, "procfs", String).unwrap(),
    );
}
