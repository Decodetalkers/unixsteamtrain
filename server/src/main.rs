use std::os::unix::net::{UnixListener, UnixStream};
use std::process::exit;
use std::thread;
use stream_message::{Message, Request, Response};

use signal_hook::{consts::SIGINT, iterator::Signals};

fn handle_client(mut stream: UnixStream) {
    loop {
        let Ok(Request::Remote { monitors }) = stream.read_msg() else {
            continue;
        };
        println!("monitors: {monitors:?}");
        let _ = stream.write_msg(&Response::Success { index: 1 });
    }
}

fn main() {
    let listener = UnixListener::bind("./server.sock").unwrap();
    let mut signals = Signals::new([SIGINT]).unwrap();
    let handle = thread::spawn(move || {
        for sig in signals.forever() {
            if sig == SIGINT {
                let _ = std::fs::remove_file("./server.sock");
                break;
            }
        }
        exit(0);
    });
    let mut threads = vec![handle];
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
