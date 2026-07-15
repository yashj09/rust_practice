use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

for stream in listener.incoming(){
    let stream = stream.unwrap();
handle_connection(stream);
    println!("connection estaliblised");
}
}
 fn handle_connection(mut stream : TcpStream){
let mut buffer = [0;1024];
stream.read(&mut buffer).unwrap();
let get = b"GET / HTTP/1.1\r\n";
    let (status_line,filename)= if buffer.starts_with(get){
        ("HTTP/1.1 200 ok", "index.html")
        } else {
            ("HTTP/1.1 200 ok", "404.html")
        };        
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents= fs::read_to_string(filename).unwrap();
        let res = format!("{}\r\nContent-Length: {} \r\n\r\n{}",status_line,contents.len(),contents);
        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();


}
