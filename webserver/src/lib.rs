mod concurrency;
pub mod config;
mod server;
mod utils;

pub fn serve(server_address: String, multi_threading: bool, number_threads: usize) {
    let server_config = config::ServerConfig::new(server_address, multi_threading, number_threads);
    server::run(server_config);
}

pub fn serve_with_config(server_config: config::ServerConfig) {
    server::run(server_config);
}
