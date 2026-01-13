use std::os::unix::net::UnixStream;
use stream_message::{Request, Response, SyncCodec};

fn main() {
    let mut stream = UnixStream::connect("server.sock").unwrap();
    Request::Remote {
        monitors: vec!["a".to_owned(), "b".to_owned()],
    }
    .write_to(&mut stream)
    .unwrap();
    let request = Response::read_from(&mut stream).unwrap();
    println!("request = {request:?}");
}
