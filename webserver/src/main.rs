use webserver::config;
use webserver::{serve, serve_with_config};

const SERVER_ADDRESS: &'static str = "127.0.0.1:7878";
const MULTI_THREADED: bool = true;
const NUMBER_THREADS: usize = 8;

fn main() {
    let server_config = config::ServerConfig::new(
            SERVER_ADDRESS.to_string(),
            MULTI_THREADED,
            NUMBER_THREADS);

    serve_with_config(server_config);
}
