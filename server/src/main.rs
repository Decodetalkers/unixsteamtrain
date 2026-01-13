use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;
use stream_message::{Request, Response, SyncCodec};

fn handle_client(mut stream: UnixStream) {
    loop {
        let Ok(Request::Remote { monitors }) = Request::read_from(&mut stream) else {
            continue;
        };
        println!("monitors: {monitors:?}");
        let _ = Response::Success { index: 1 }.write_to(&mut stream);
    }
}

fn main() {
    let _ = std::fs::remove_file("./server.sock");
    let listener = UnixListener::bind("./server.sock").unwrap();
    let mut threads = vec![];
    for stream in listener.incoming() {
        let Ok(stream) = stream else {
            break;
        };
        let handler = thread::spawn(|| handle_client(stream));
        threads.push(handler);
    }
    for thread in threads {
        let _ = thread.join();
    }
    println!("Hello, world!");
}
