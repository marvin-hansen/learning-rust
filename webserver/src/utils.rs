use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let (status_line, filename) = get_status_file_name(buf_reader);
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

pub fn get_status_file_name(buf_reader: BufReader<&mut TcpStream>) -> (String, String) {
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        (String::from("HTTP/1.1 200 OK"), String::from("index.html"))
    } else {
        (
            String::from("HTTP/1.1 404 NOT FOUND"),
            String::from("404.html"),
        )
    };
    return (status_line, filename);
}
