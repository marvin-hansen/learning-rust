mod utils;

use std::net::{TcpListener};

use concurrency::ThreadPool;
use config::{ServerConfig, ServerType};

pub fn serve(server_address: String, multi_threading: bool) {
    let server_config = ServerConfig::new(server_address, multi_threading);
    run(server_config);
}

fn run(server_config: ServerConfig) {
    match server_config.get_server_type() {
        ServerType::SingleThreaded => {
            serve_single_threaded(server_config.get_server_address());
        }
        ServerType::MultiThreaded => {
            serve_multi_threaded(server_config.get_server_address());
        }
    }
}

pub fn serve_single_threaded(address: String) {
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();


        utils::handle_connection(stream);


    }
}

pub fn serve_multi_threaded(address: String) {
    let listener = TcpListener::bind(address).unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();


        pool.execute(|| utils::handle_connection(stream));


    }
}



mod concurrency {
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;

    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<Job>,
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    impl ThreadPool {
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0, "Pool size must be positive");

            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers: Vec<Worker> = Vec::with_capacity(size);

            for id in 0..size {
                // Create new workers to execute handlers
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            ThreadPool { workers, sender }
        }

        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.send(job).unwrap();
        }
    }

    struct Worker {
        id: usize,
        thread: thread::JoinHandle<()>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} got a job; executing.");

                job();
            });

            Worker { id, thread }
        }
    }
}

mod config {
    #[derive(Debug, Clone, PartialEq)]
    pub enum ServerType {
        SingleThreaded,
        MultiThreaded,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ServerConfig {
        server_type: ServerType,
        server_addr: String,
    }

    impl ServerConfig {
        pub fn new(server_addr: String, multi_threading: bool) -> ServerConfig {
            assert!(
                server_addr.len() > 0,
                "Address empty. Please specify a server address"
            );

            let server_type: ServerType; // Deferred initialization
            match multi_threading {
                true => server_type = ServerType::SingleThreaded,
                false => server_type = ServerType::MultiThreaded,
            }

            return ServerConfig {
                server_type,
                server_addr,
            };
        }

        pub fn get_server_type(&self) -> ServerType {
            return self.server_type.clone();
        }

        pub fn get_server_address(&self) -> String {
            return self.server_addr.clone();
        }
    }
}
