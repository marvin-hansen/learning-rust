use webserver::serve;

const SERVER_ADDRESS: &'static str = "127.0.0.1:7878";
const MULTI_THREADED: bool = false;

fn main() {
    serve(SERVER_ADDRESS.to_string(), MULTI_THREADED)
}
