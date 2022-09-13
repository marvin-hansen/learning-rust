#[derive(Debug, Clone, PartialEq)]
pub enum ServerType {
    SingleThreaded,
    MultiThreaded,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServerConfig {
    server_type: ServerType,
    server_addr: String,
    number_threads: usize,
}

impl ServerConfig {
    pub fn new(server_addr: String, multi_threading: bool, number_threads: usize) -> ServerConfig {
        assert!(
            server_addr.len() > 0,
            "Address empty. Please specify a server address"
        );

        if multi_threading {
            assert!(
                number_threads > 0,
                "Number of threads must be greater than zero"
            )
        }

        let server_type: ServerType; // Deferred initialization
        match multi_threading {
            true => server_type = ServerType::MultiThreaded,
            false => server_type = ServerType::SingleThreaded,
        }

        return ServerConfig {
            server_type,
            server_addr,
            number_threads,
        };
    }

    pub fn get_server_type(&self) -> ServerType {
        return self.server_type.clone();
    }

    pub fn get_server_address(&self) -> String {
        return self.server_addr.clone();
    }

    pub fn get_number_threads(&self) -> usize {
        return self.number_threads.clone();
    }
}
