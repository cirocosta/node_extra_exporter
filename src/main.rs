extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("node_extra_exporter")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .args(&[
            Arg::with_name("address")
                .default_value("0.0.0.0")
                .short("a")
                .long("address")
                .help("IP address to listen for Prometheus requests"),
            Arg::with_name("port")
                .default_value("9001")
                .short("p")
                .long("port")
                .help("Port to listen for Prometheus requests"),
            Arg::with_name("procfs")
                .default_value("/proc")
                .long("procfs")
                .help("Location where `procfs` is mounted to"),
        ])
        .get_matches();
}
