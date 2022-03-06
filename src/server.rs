use std::collections::HashMap;
use std::io::{Error, Read};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

fn main() -> Result<(), Error> {
    let mut h: HashMap<String, usize> = HashMap::new();

    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 31337);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;

    println!("Listening on {}", port);

    loop {
        // Call to accept blocks until requested
        let (mut tcp_stream, _addr) = listener.accept()?; 

        let mut input = String::new();
        let _ = tcp_stream.read_to_string(&mut input)?;

        // Update the counter accordingly
        let counter = h.entry(input).or_insert(0);
        *counter += 1;

        // Clear the terminal
        print!("\x1B[2J\x1B[1;1H");

        for (key, value) in &h {
            println!("{}: {}", key, value);
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}
