use std::net::{TcpStream,TcpListener};
use std::io::Read;
use tracing::{debug, info, instrument};

mod route;

#[instrument]
fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
    debug!("handle_client beginning");

    let mut buff = [0;1024];

    let bytes_read = stream.read(&mut buff)?;
    let raw_request = String::from_utf8_lossy( &buff[..bytes_read] );
    debug!(%raw_request, "raw_request received from client");
    let parsed_request = route::parse_request(&raw_request);

    route::route_request(&parsed_request)?;
    // println!("parsed_request={parsed_request}");
    Ok(())
}



fn main() -> anyhow::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())

}
