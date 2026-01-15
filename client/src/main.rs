use std::{
    os::unix::net::UnixStream,
    time::Duration,
};
use stream_message::{Request, Response};

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("./server.sock")?;

    // Optional: avoid hanging forever
    stream.set_read_timeout(Some(Duration::from_secs(30)))?;

    stream.write_msg(Request::Remote {
        monitors: vec!["a".into(), "b".into()],
    })?;

    match stream.read_msg::<Response>() {
        Ok(Response::Success { index }) => {
            println!("Selected index: {index}");
        }
        Ok(Response::Cancel) => {
            println!("User rejected selection");
        }
        Ok(Response::Busy) => {
            println!("Server is busy, try again later");
        }
        Err(e) => {
            eprintln!("Failed");
            return Err(e);
        }
    }

    ok(());
}