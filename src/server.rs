use std::net::SocketAddr;

use crate::schedstat::collect_system_schedstat;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};

fn handle_metrics(procfs: &str) -> Response<Body> {
    let stats = match collect_system_schedstat(&procfs) {
        Err(err) => panic!("failed to collect statistics - {}", err),
        Ok(stats) => stats,
    };

    Response::new(Body::from(
        stats
            .iter()
            .enumerate()
            .map(|(idx, stat)| stat.to_prometheus_samples(idx))
            .collect::<String>(),
    ))
}

pub fn serve(address: &str, procfs: String) {
    let addr: SocketAddr = address.parse().unwrap();

    let metrics_svc = move || {
        let procfs = procfs.clone();

        service_fn_ok(move |_| handle_metrics(&procfs))
    };

    let server = Server::bind(&addr)
        .serve(metrics_svc)
        .map_err(|e| println!("server error: {}", e));

    println!("listening on {}", address);

    hyper::rt::run(server);
}
