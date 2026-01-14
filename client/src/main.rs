use std::os::unix::net::UnixStream;
use stream_message::{Message, Request, Response};

fn main() {
    let mut stream = UnixStream::connect("server.sock").unwrap();
    stream
        .write_msg(&Request::Remote {
            monitors: vec!["a".to_owned(), "b".to_owned()],
        })
        .unwrap();

    let request: Response = stream.read_msg().unwrap();
    println!("request = {request:?}");
}
