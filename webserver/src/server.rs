use crate::{concurrency, config, utils};
use std::net::TcpListener;

pub fn run(server_config: config::ServerConfig) {
    match server_config.get_server_type() {
        config::ServerType::SingleThreaded => {
            serve_single_threaded(server_config.get_server_address());
        }
        config::ServerType::MultiThreaded => {
            serve_multi_threaded(
                server_config.get_server_address(),
                server_config.get_number_threads(),
            );
        }
    }
}

fn serve_single_threaded(address: String) {
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        utils::handle_connection(stream);
    }
}

fn serve_multi_threaded(address: String, size: usize) {
    let listener = TcpListener::bind(address).unwrap();
    let pool = concurrency::ThreadPool::new(size);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| utils::handle_connection(stream));
    }
}
