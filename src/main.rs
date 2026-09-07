use std::net::{TcpStream,TcpListener};
use std::io::Read;

mod route;

fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {

    let mut buff = [0;1024];

    let bytes_read = stream.read(&mut buff)?;
    let raw_request = String::from_utf8_lossy( &buff[..bytes_read] );
    println!("raw_request={raw_request}");
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
